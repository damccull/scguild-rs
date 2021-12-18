use actix_web::{http::header, web, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use twilight_model::application::{
    callback::InteractionResponse,
    interaction::{ApplicationCommand, Interaction},
};

use crate::error_chain_fmt;

use super::commands::{FleetCommand, HelloCommand};

#[tracing::instrument(name = "Calling Discord API", skip(_req, interaction))]
pub async fn discord_api(
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    interaction: web::Json<Interaction>,
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
            let response = application_command_handler(&c)
                .await
                .context("Problem running application command handler")?;
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

#[tracing::instrument(name = "Handling ApplicationCommand", skip(cmd))]
async fn application_command_handler(
    cmd: &ApplicationCommand,
) -> Result<InteractionResponse, DiscordApiError> {
    match cmd.data.name.as_ref() {
        FleetCommand::NAME => FleetCommand::handler(cmd).await,
        HelloCommand::NAME => HelloCommand::handler(cmd).await,
        _ => Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
            "Invalid interaction data".to_string()
        ))),
    }
}

#[tracing::instrument(name = "Handling ApplicationCommandAutocomplete", skip(cmd))]
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
