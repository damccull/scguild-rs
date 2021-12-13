use actix_web::{http::header, web, HttpRequest, HttpResponse, ResponseError};
use anyhow::Context;
use twilight_interactions::command::{CommandInputData, CommandModel};
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    interaction::Interaction,
};

use crate::error_chain_fmt;

use super::{
    commands::{About, Fleet, HelloCommand, Wishlist},
    SlashCommand,
};

#[tracing::instrument(name = "Calling Discord API", skip(_req, interaction))]
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

#[tracing::instrument(name = "Handling ApplicationCommand", skip(interaction))]
async fn application_command_handler(
    interaction: Interaction,
) -> Result<InteractionResponse, DiscordApiError> {
    match interaction {
        Interaction::ApplicationCommand(ref cmd) => match cmd.data.name.as_ref() {
            About::NAME => About::api_handler(cmd).await,
            Fleet::NAME => Fleet::api_handler(cmd).await,
            "hello" => {
                let x: CommandInputData = cmd.data.clone().into();
                let x: HelloCommand = HelloCommand::from_interaction(x).unwrap();
                let nick = match x.user {
                    Some(ref y) => match y.member.clone() {
                        Some(z) => z.nick.unwrap_or_else(|| y.resolved.name.to_owned()),
                        None => y.resolved.name.to_owned(),
                    },
                    None => "everyone".into(),
                };
                let message = x.message;
                Ok(InteractionResponse::ChannelMessageWithSource(
                    CallbackData {
                        allowed_mentions: None,
                        flags: None,
                        tts: None,
                        content: Some(format!("{}, {}", message, nick)),
                        embeds: Default::default(),
                        components: Default::default(),
                    },
                ))
            }
            Wishlist::NAME => Wishlist::api_handler(cmd).await,
            _ => Err(DiscordApiError::UnsupportedInteraction(interaction)),
        },
        _ => Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
            "Invalid interaction data".to_string()
        ))),
    }
}

#[derive(thiserror::Error)]
pub enum DiscordApiError {
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
            DiscordApiError::UnsupportedInteraction(_) | DiscordApiError::UnsupportedCommand(_) => {
                actix_http::StatusCode::BAD_REQUEST
            }

            DiscordApiError::UnexpectedError(_) => actix_http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
