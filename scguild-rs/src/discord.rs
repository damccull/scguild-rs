use std::fmt::Display;

use serde::{Deserialize, Serialize};
use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::{Command, CommandOption};

use crate::discord::commands::{FleetCommand, HelloCommand};

pub mod api;
mod commands;
pub mod twilight_interactions_extensions;

pub fn commands() -> Vec<Command> {
    vec![
        // About::define(),
        FleetCommand::register(),
        // Wishlist::define(),
        HelloCommand::create_command().into(),
    ]
}

pub trait DiscordCommand {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;

    fn register() -> Command;
}

pub trait DiscordSubcommand {
    const NAME: &'static str;
    const DESCRIPTION: &'static str;

    fn register() -> CommandOption;
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
