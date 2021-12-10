use actix_web::{http::header, web, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    interaction::Interaction,
};

use crate::error_chain_fmt;

use super::{commands::About, SlashCommand};

#[tracing::instrument(name = "calling discord api", skip(_req, interaction))]
pub async fn discord_api(
    _req: HttpRequest,
    interaction: web::Json<Interaction>,
) -> Result<HttpResponse, DiscordApiError> {
    match interaction.0 {
        Interaction::Ping(_) => Ok(HttpResponse::Ok()
            .append_header(header::ContentType(mime::APPLICATION_JSON))
            .json(InteractionResponse::Pong)),
        Interaction::ApplicationCommand(_) => {
            // Run handler to get correct response
            let response = application_command_handler(interaction.0)
                .await
                .context("Problem running application command handler")?;
            Ok(HttpResponse::Ok()
                .append_header(header::ContentType(mime::APPLICATION_JSON))
                .json(response))
        }
        _ => Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
            "Bad interaction".to_string()
        ))),
    }
}

#[tracing::instrument(name = "application_command_handler", skip(interaction))]
async fn application_command_handler(
    interaction: Interaction,
) -> Result<InteractionResponse, DiscordApiError> {
    match interaction {
        Interaction::ApplicationCommand(ref cmd) => match cmd.data.name.as_ref() {
            About::NAME => About::about(interaction).await,
            "debug" => debug(interaction).await,
            _ => debug(interaction).await,
        },
        _ => Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
            "Invalid interaction data".to_string()
        ))),
    }
}

#[tracing::instrument(name = "Discord Interaction - DEBUG")]
async fn debug(interaction: Interaction) -> Result<InteractionResponse, DiscordApiError> {
    Ok(InteractionResponse::ChannelMessageWithSource(
        CallbackData {
            allowed_mentions: None,
            flags: None,
            tts: None,
            content: Some(format!("```rust\n{:?}\n```", interaction)),
            embeds: Default::default(),
            components: Default::default(),
        },
    ))
}

#[derive(thiserror::Error)]
pub enum DiscordApiError {
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
            DiscordApiError::UnexpectedError(_) => actix_http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
