use std::{net::TcpListener, sync::Arc};

use actix_cors::Cors;
use actix_web::{dev::Server, web::Data, App, HttpServer};
use anyhow::Context;
use reqwest::header;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_actix_web::TracingLogger;

use ed25519_dalek::PublicKey;
use twilight_http::Client as TwilightHttpClient;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    configuration::{DatabaseSettings, DiscordSettings, Settings},
    discord, webapp,
};

pub struct Application {
    port: u16,
    server: Server,
    discord_settings: DiscordSettings,
}
impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        // Get a connection pool for the database
        let connection_pool = get_connection_pool(&configuration.database);

        // Get and store the application's host and port
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        // Create a TCP listener
        let listener = TcpListener::bind(address)?;

        // Store the listener's actual port for future use
        let port = listener.local_addr().unwrap().port();

        // Store the discord settings in the app for future use
        let discord_settings = configuration.discord.clone();

        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            configuration.discord.public_key,
        )?;

        Ok(Self {
            port,
            server,
            discord_settings,
        })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    #[tracing::instrument(name = "Registering commands with discord", skip(self))]
    pub async fn register_commands_with_discord(&self) -> Result<(), anyhow::Error> {
        let access_token = self.get_discord_auth_token().await?;

        tracing::debug!("Setting up twilight http client.");
        let http = TwilightHttpClient::new(format!("Bearer {}", access_token));

        tracing::debug!("Getting application_id");
        let application_id = http
            .current_user_application()
            .exec()
            .await
            .context("Unable to get application.")?
            .model()
            .await
            .context("Unable to get model.")?
            .id;

        tracing::debug!("Setting test guild ID.");
        let guild_id = Id::<GuildMarker>::new(self.discord_settings.guild_id);

        tracing::debug!("Setting guild commands.");
        http.interaction(application_id)
            .set_guild_commands(guild_id, &discord::commands())
            .exec()
            .await
            .context("Unable to set_guild_commands")?;

        tracing::info!("Guild commands registered.");

        // http.set_application_id(ApplicationId::new(self.discord_settings.application_id).unwrap());

        // // // http.set_global_commands(&discord_commands::commands())?
        // // //     .exec()
        // // //     .await?;
        // http.set_guild_commands(
        //     GuildId::new(self.discord_settings.guild_id).unwrap(),
        //     &discord::commands(),
        // )?
        // .exec()
        // .await?;

        // REMOVE COMMANDS
        // http.set_global_commands(&[])?.exec().await?;
        // http.set_guild_commands(GuildId::new(745809834183753828).unwrap(), &[])?
        //     .exec()
        //     .await?;
        Ok(())
    }

    #[tracing::instrument(name = "Get Discord Auth Token", skip(self))]
    async fn get_discord_auth_token(&self) -> anyhow::Result<String> {
        #[derive(Debug, Deserialize)]
        struct ClientCredential {
            #[serde(rename = "access_token")]
            pub access_token: String,
            #[serde(rename = "expires_in")]
            pub _expires_in: u64,
            #[serde(rename = "scope")]
            pub _scope: String,
            #[serde(rename = "token_type")]
            pub _token_type: String,
        }
        let reqwestclient = reqwest::Client::new();

        tracing::debug!("Getting client credential.");

        let params = [
            ("grant_type", "client_credentials"),
            (
                "scope",
                "applications.commands applications.commands.update",
            ),
        ];
        let client_credential = reqwestclient
            .post("https://discord.com/api/oauth2/token")
            .header(
                header::CONTENT_TYPE,
                mime::APPLICATION_WWW_FORM_URLENCODED.to_string(),
            )
            .basic_auth(
                &self.discord_settings.client_id,
                Some(&self.discord_settings.client_secret),
            )
            .form(&params)
            .send()
            .await
            .context("Error requesting client credential from Discord API")?;

        tracing::debug!("Deserializing client credential.");

        let client_credential = client_credential
            .json::<ClientCredential>()
            .await
            .context("Error deserializing client credential")?;

        tracing::debug!("Client credential is valid.");
        tracing::debug!("Access token is {}", &client_credential.access_token);
        Ok(client_credential.access_token)
    }
}

/// Returns a `Server` without awaiting it. This allows for integration testing.
///
/// Takes a `TcpListener`, expecting it to already be bound. This allows for easy integration testing.
fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    discord_public_key: PublicKey,
) -> Result<Server, std::io::Error> {
    // Wrap shared things in smart pointers
    let base_url = Data::new(base_url);
    let db_pool = Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(TracingLogger::default())
            .configure(webapp::health_check::configure)
            .configure(discord::api::configure(discord_public_key))
            .configure(webapp::api::configure)
            .app_data(base_url.clone())
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

/// Returns a `PgPool`
///
/// Public so that the integration tests can use this too.
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
