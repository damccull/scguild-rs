use async_trait::async_trait;
use twilight_model::application::{
    callback::InteractionResponse,
    command::Command,
    interaction::{ApplicationCommand, Interaction},
};

use self::{
    api::DiscordApiError,
    commands::{About, Fleet},
};

pub mod api;
mod commands;
// mod interaction;

#[async_trait]
trait SlashCommand {
    /// Name of the command.
    /// Required to match incoming interactions.
    const NAME: &'static str;

    /// Command definition
    fn define() -> Command;

    async fn api_handler(interaction: Interaction) -> Result<InteractionResponse, DiscordApiError>;
}

pub fn commands() -> Vec<Command> {
    vec![
        About::define(),
        Fleet::define(),
        // Ships::define(),
        // AddShip::define,
        // RemoveShip::define(),
    ]
}
