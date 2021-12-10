use std::{net::TcpListener, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    dev::Server,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_actix_web::TracingLogger;

use ed25519_dalek::PublicKey;
use twilight_http::Client as HttpClient;
use twilight_model::id::GuildId;

use crate::{
    configuration::{DatabaseSettings, Settings},
    discord::{self, api::discord_api},
    middleware::ed25519_signatures,
    webapp::{api, health_check},
};

pub struct Application {
    port: u16,
    server: Server,
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

        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            configuration.discord.public_key,
        )?;

        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
    pub async fn register_commands_with_discord(&self) -> Result<(), anyhow::Error> {
        // TODO: Use reqwest to get a Client Credentials grant to retrieve a discord token

        let token = std::env::var("DISCORD_TOKEN")?;

        let http = Arc::new(HttpClient::new(token.clone()));

        let current_user = http
            .current_user_application()
            .exec()
            .await?
            .model()
            .await?;
        http.set_application_id(current_user.id.0.into());

        // http.set_global_commands(&discord_commands::commands())?
        //     .exec()
        //     .await?;
        http.set_guild_commands(
            GuildId::new(745809834183753828).unwrap(),
            &discord::commands(),
        )?
        .exec()
        .await?;

        // REMOVE COMMANDS
        // http.set_global_commands(&[])?.exec().await?;
        // http.set_guild_commands(GuildId::new(745809834183753828).unwrap(), &[])?
        //     .exec()
        //     .await?;
        Ok(())
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
    // let db_pool = Data::new(db_pool);
    let base_url = Data::new(base_url);

    // let db_pool = Data::new(db_pool);
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
            .route("/health_check", web::get().to(health_check))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/discord")
                            .wrap(ed25519_signatures::VerifyEd25519Signature::new(
                                discord_public_key,
                            ))
                            .route(web::post().to(discord_api)),
                    )
                    .route("/v1/{interaction}", web::get().to(api)),
            )
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
