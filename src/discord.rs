use std::{convert::TryFrom, fmt::Display, num::NonZeroU64};

use serde::{Deserialize, Serialize};
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{
        callback::{CallbackData, InteractionResponse},
        command::Command,
    },
    id::UserId,
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

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type)]
pub struct DiscordUserId(NonZeroU64);
impl Display for DiscordUserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
impl TryFrom<i64> for DiscordUserId {
    type Error = anyhow::Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match NonZeroU64::new(value as u64) {
            Some(x) => Ok(Self { 0: x }),
            None => Err(anyhow::anyhow!("Unable to parse user id.")),
        }
    }
}
impl TryFrom<&str> for DiscordUserId {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let i = match value.parse::<u64>() {
            Ok(x) => x,
            Err(_) => return Err(anyhow::anyhow!("Unable to parse user id.")),
        };
        match NonZeroU64::new(i) {
            Some(x) => Ok(Self { 0: x }),
            None => Err(anyhow::anyhow!("Unable to parse user id.")),
        }
    }
}
impl From<UserId> for DiscordUserId {
    fn from(x: UserId) -> Self {
        Self { 0: x.0 }
    }
}
impl Into<UserId> for DiscordUserId {
    fn into(self) -> UserId {
        UserId { 0: self.0 }
    }
}
