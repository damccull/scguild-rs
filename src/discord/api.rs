use actix_web::{http::header, web, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use tracing_actix_web::RequestId;
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    interaction::{ApplicationCommand, Interaction},
};

use crate::error_chain_fmt;

use super::commands::{FleetCommand, HelloCommand};

#[tracing::instrument(name = "Calling Discord API", skip(_req, interaction))]
pub async fn discord_api(
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    interaction: web::Json<Interaction>,
    request_id: RequestId,
) -> Result<HttpResponse, DiscordApiError> {
    let interaction = interaction.into_inner();
    match interaction {
        Interaction::Ping(_) => {
            tracing::info!("Received ping, sending pong.");
            Ok(HttpResponse::Ok()
                .append_header(header::ContentType(mime::APPLICATION_JSON))
                .json(InteractionResponse::Pong))
        }
        Interaction::ApplicationCommand(c) => {
            // Run handler to get correct response
            let response = application_command_handler(&c, &pool).await;
            // .context("Problem running application command handler");

            //TODO: If response is Err, log the error, send a pretty display to the user

            let response = match response {
                Ok(response) => response,
                Err(e) => {
                    tracing::error!("An error occurred: {:?}", e);
                    format_user_error(request_id).await
                }
            };

            Ok(HttpResponse::Ok()
                .append_header(header::ContentType(mime::APPLICATION_JSON))
                .json(response))
        }
        Interaction::ApplicationCommandAutocomplete(c) => {
            let response = application_command_autocomplete_handler(&c, &pool)
                .await
                .context("Problem running application command autocomplete handler")?;

            Ok(HttpResponse::Ok()
                .append_header(header::ContentType(mime::APPLICATION_JSON))
                .json(response))
        }
        _ => Err(DiscordApiError::UnsupportedInteraction(interaction)),
    }
}

async fn format_user_error(request_id: RequestId) -> InteractionResponse {
    let body = format!(
        "Request ID: {}\n\n\
        What were you doing when the error occurred? Please provide as much detail as possible, \
        including the command you typed, if possible.\n",
        request_id
    );

    let body = urlencoding::encode(&body);

    InteractionResponse::ChannelMessageWithSource(CallbackData {
        allowed_mentions: None,
        flags: None,
        tts: None,
        content: Some(format!(
            "There was an error processing your request. \
            If this happens repeatedly, \
            [please open an issue on the github repo](<https://github.com/damccull/norseline-rs/issues/new?body={}>) \
            with this request id: {}",
            body, request_id
        )),
        embeds: Default::default(),
        components: Default::default(),
    })
}

#[tracing::instrument(name = "Handling ApplicationCommand", skip(cmd, pool))]
async fn application_command_handler(
    cmd: &ApplicationCommand,
    pool: &PgPool,
) -> Result<InteractionResponse, DiscordApiError> {
    match cmd.data.name.as_ref() {
        FleetCommand::NAME => FleetCommand::handler(cmd, pool).await,
        HelloCommand::NAME => HelloCommand::handler(cmd).await,
        _ => Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
            "Invalid interaction data".to_string()
        ))),
    }
}

#[tracing::instrument(name = "Handling ApplicationCommandAutocomplete", skip(cmd, pool))]
async fn application_command_autocomplete_handler(
    cmd: &ApplicationCommand,
    pool: &PgPool,
) -> Result<InteractionResponse, DiscordApiError> {
    match cmd.data.name.as_ref() {
        FleetCommand::NAME => FleetCommand::autocomplete_handler(cmd, pool).await,
        _ => Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
            "Invalid autocomplete interaction data".to_string()
        ))),
    }
}

#[derive(thiserror::Error)]
pub enum DiscordApiError {
    #[error("Autocomplete is not supported for this command.")]
    AutocompleteUnsupported,
    #[error("Unsupported interaction: {0:?}")]
    UnsupportedInteraction(Interaction),
    #[error("Unsupported command: {0:?}")]
    UnsupportedCommand(String),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}
// impl std::fmt::Display for DiscordApiError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format!("There as an error processing your request."))
//     }
// }
impl std::fmt::Debug for DiscordApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
impl ResponseError for DiscordApiError {
    fn status_code(&self) -> actix_http::StatusCode {
        match self {
            DiscordApiError::AutocompleteUnsupported
            | DiscordApiError::UnsupportedInteraction(_)
            | DiscordApiError::UnsupportedCommand(_) => actix_http::StatusCode::BAD_REQUEST,

            DiscordApiError::UnexpectedError(_) => actix_http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
