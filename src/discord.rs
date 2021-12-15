use async_trait::async_trait;
use twilight_interactions::command::CreateCommand;
use twilight_model::application::{
    callback::InteractionResponse, command::Command, interaction::ApplicationCommand,
};

use crate::discord::commands::{FleetCommand, HelloCommand};

use self::api::DiscordApiError;

pub mod api;
mod commands;
// mod interaction;

#[async_trait]
trait SlashCommand {
    async fn handler(cmd: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError>;
    async fn autocomplete_handler(
        cmd: &ApplicationCommand,
    ) -> Result<InteractionResponse, DiscordApiError>;
}

pub fn commands() -> Vec<Command> {
    vec![
        // About::define(),
        FleetCommand::create_command().into(),
        // Wishlist::define(),
        HelloCommand::create_command().into(),
    ]
}
