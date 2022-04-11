use std::fmt::Display;

use serde::{Deserialize, Serialize};
use twilight_interactions::command::CreateCommand;
use twilight_model::{application::command::Command, http::interaction::InteractionResponseData};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::discord::commands::{FleetCommand, HelloCommand};

pub mod api;
pub mod twilight_interactions_extensions;
mod commands;

pub fn commands() -> Vec<Command> {
    vec![
        // About::define(),
        FleetCommand::register().into(),
        // Wishlist::define(),
        HelloCommand::create_command().into(),
    ]
}

pub fn format_simple_message_response(message: &str) -> InteractionResponseData {
    InteractionResponseDataBuilder::new()
        .content(message.to_string())
        .build()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiscordId<T>(T);
impl<T: Display> Display for DiscordId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct DiscordUserId(i64);
// impl Display for DiscordUserId {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
// impl From<UserId> for DiscordUserId {
//     fn from(id: UserId) -> Self {
//         DiscordUserId(u64::from(id.0) as i64)
//     }
// }
// impl TryFrom<i64> for DiscordUserId {
//     type Error = anyhow::Error;

//     fn try_from(value: i64) -> Result<Self, Self::Error> {
//         match NonZeroU64::new(value as u64) {
//             Some(x) => Ok(Self(x)),
//             None => Err(anyhow::anyhow!("Unable to parse user id.")),
//         }
//     }
// }
// impl TryFrom<&str> for DiscordUserId {
//     type Error = anyhow::Error;

//     fn try_from(value: &str) -> Result<Self, Self::Error> {
//         let i = match value.parse::<u64>() {
//             Ok(x) => x,
//             Err(_) => return Err(anyhow::anyhow!("Unable to parse user id.")),
//         };
//         match NonZeroU64::new(i) {
//             Some(x) => Ok(Self(x)),
//             None => Err(anyhow::anyhow!("Unable to parse user id.")),
//         }
//     }
// }
// impl From<UserId> for DiscordUserId {
//     fn from(x: UserId) -> Self {
//         Self(x.0)
//     }
// }

// #[allow(clippy::from_over_into)]
// impl Into<UserId> for DiscordUserId {
//     fn into(self) -> UserId {
//         UserId(self.0)
//     }
// }

// // https://discord.com/channels/665528275556106240/694697474689859614/923193721175035945

// impl sqlx::Type<sqlx::Postgres> for DiscordUserId {
//     fn type_info() -> sqlx::postgres::PgTypeInfo {
//         <i64 as sqlx::Type<sqlx::Postgres>>::type_info()
//     }

//     fn compatible(ty: &sqlx::postgres::PgTypeInfo) -> bool {
//         <i64 as sqlx::Type<sqlx::Postgres>>::compatible(ty)
//     }
// }
