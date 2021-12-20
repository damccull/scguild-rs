use twilight_interactions::command::CreateCommand;
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    command::Command,
};

use crate::discord::commands::{FleetCommand, HelloCommand};

pub mod api;
mod commands;

pub fn commands() -> Vec<Command> {
    vec![
        // About::define(),
        FleetCommand::create_command().into(),
        // Wishlist::define(),
        HelloCommand::create_command().into(),
    ]
}

pub fn format_simple_message_response(message: &str) -> InteractionResponse {
    InteractionResponse::ChannelMessageWithSource(CallbackData {
        allowed_mentions: None,
        flags: None,
        tts: None,
        content: Some(message.to_string()),
        embeds: Default::default(),
        components: Default::default(),
    })
}
