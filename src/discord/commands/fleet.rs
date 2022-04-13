use std::convert::TryInto;

use crate::discord::{
    api::DiscordApiError, commands::list::ListCommand, DiscordCommand, DiscordSubcommand,
};
use sqlx::PgPool;

use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::{ApplicationCommand, ApplicationCommandAutocomplete},
    },
    http::interaction::InteractionResponseData,
};
use twilight_util::builder::command::CommandBuilder;
use uuid::Uuid;

use self::add::AddCommand;

pub mod add;
pub mod list;

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub struct FleetCommand;
impl DiscordCommand for FleetCommand {
    const NAME: &'static str = "fleet";
    const DESCRIPTION: &'static str = "Manage or view your fleet, or show it off.";

    fn register() -> Command {
        CommandBuilder::new(
            Self::NAME.into(),
            Self::DESCRIPTION.into(),
            CommandType::ChatInput,
        )
        .option(AddCommand::register())
        .option(ListCommand::register())
        .build()
    }
}

impl FleetCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET DISPATCH", skip(pool))]
    pub async fn handler(
        cmd: &ApplicationCommand,
        pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        let command_name = cmd.data.name.as_str();
        tracing::debug!("Command Name: {}", command_name);

        let subcommand = cmd.data.options.clone().pop().ok_or_else(|| {
            DiscordApiError::UnexpectedError(anyhow::anyhow!("Subcommand missing."))
        })?;
        tracing::debug!("Subcommand Name: {}", subcommand.name);

        match subcommand.name.as_str() {
            AddCommand::NAME => {
                let add_command: AddCommand = cmd.data.options.clone().try_into()?;
                add_command.handler(cmd, pool).await
            }
            ListCommand::NAME => {
                todo!()
                // let list_command: ListCommand = cmd.data.options.clone().try_into()?;
                // list_command.handler(cmd, pool).await
            }
            // FleetCommand::List(list_command) => list_command.handler(cmd, pool).await,
            // FleetCommand::Remove(_) => todo!(),
            // FleetCommand::Rename(_) => todo!(),
            // FleetCommand::Show(show_command) => show_command.handler(cmd, pool).await,
            _ => {
                return Err(DiscordApiError::UnsupportedCommand(format!(
                    "No subcommand '{}' exists.",
                    cmd.data.name
                )));
            }
        }
    }

    #[tracing::instrument(name = "Discord Interaction - FLEET AUTOCOMPLETE DISPATCH", skip(pool))]
    pub async fn autocomplete_handler(
        autocomplete: &ApplicationCommandAutocomplete,
        pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        let command_name = autocomplete.data.name.as_str();
        tracing::debug!("Command Name: {}", command_name);

        let subcommand = autocomplete.data.options.clone().pop().ok_or_else(|| {
            DiscordApiError::UnexpectedError(anyhow::anyhow!("Subcommand missing."))
        })?;

        match subcommand.name.as_str() {
            AddCommand::NAME => {
                let add_command: AddCommand = autocomplete.data.options.clone().try_into()?;
                add_command.autocomplete_handler(autocomplete, pool).await
            }
            _ => Err(DiscordApiError::UnsupportedCommand(
                "Autocomplete not supported on command.".to_string(),
            )),
        }
    }
}

// #[derive(CommandModel, CreateCommand, Debug)]
// #[command(
//     name = "list",
//     desc = "Privately list the ships in your, or the specified user's, fleet."
// )]
// pub struct ListCommand {
//     /// The user who's fleet you'd like to see. (Optional)
//     pub user: Option<ResolvedUser>,
// }

// impl ListCommand {
//     #[tracing::instrument(name = "Discord Interaction - FLEET LIST", skip(_pool))]
//     async fn handler(
//         &self,
//         _cmd: &ApplicationCommand,
//         _pool: &PgPool,
//     ) -> Result<InteractionResponseData, DiscordApiError> {
//         Ok(format_simple_message_response(
//             "Privately perusing the fleet.",
//         ))
//     }
// }

// #[derive(CommandModel, CreateCommand, Debug)]
// #[command(name = "remove", desc = "Remove a ship from your fleet.")]
// pub struct RemoveCommand {}

// impl RemoveCommand {
//     #[tracing::instrument(name = "Discord Interaction - FLEET REMOVE", skip(_pool))]
//     async fn handler(
//         _cmd: &ApplicationCommand,
//         _pool: &PgPool,
//     ) -> Result<InteractionResponseData, DiscordApiError> {
//         Ok(format_simple_message_response(
//             "Removing a ship from the fleet.",
//         ))
//     }
// }

// #[derive(CommandModel, CreateCommand, Debug)]
// #[command(name = "rename", desc = "Remame a ship in your fleet.")]
// pub struct RenameCommand {}

// impl RenameCommand {
//     #[tracing::instrument(name = "Discord Interaction - FLEET RENAME", skip(_pool))]
//     async fn handler(
//         _cmd: &ApplicationCommand,
//         _pool: &PgPool,
//     ) -> Result<InteractionResponseData, DiscordApiError> {
//         Ok(format_simple_message_response(
//             "Renaming a ship in the fleet.",
//         ))
//     }
// }

// #[derive(CommandModel, CreateCommand, Debug)]
// #[command(name = "show", desc = "Show your fleet to the channel.")]
// pub struct ShowCommand;

// impl ShowCommand {
//     #[tracing::instrument(name = "Discord Interaction - FLEET SHOW", skip(_pool))]
//     async fn handler(
//         &self,
//         _cmd: &ApplicationCommand,
//         _pool: &PgPool,
//     ) -> Result<InteractionResponseData, DiscordApiError> {
//         unsafe {
//             Ok(format_simple_message_response(&format!(
//                 "Showing off the fleet.\n```\n{:?}\n```",
//                 FAKEDB
//             )))
//         }
//     }
// }

// AUTOCOMPLETE command models

// #[derive(CommandModel, Debug)]
// #[command(partial = true)]
// pub enum FleetCommandPartial {
//     #[command(name = "add")]
//     Add(AddCommandPartial),
// }

// #[derive(CommandModel, Debug)]
// #[command(partial = true)]
// pub struct AddCommandPartial {
//     /// The model of ship you want to add.
//     #[command(rename = "model")]
//     pub ship_model: String,
// }

//TODO: REENABLE
// impl AddCommandPartial {
//     #[tracing::instrument(name = "Discord Autocomplete Handler - AddCommandPartial", skip(pool))]
//     async fn handle(
//         &self,
//         _cmd: &ApplicationCommand,
//         pool: &PgPool,
//     ) -> Result<InteractionResponseData, DiscordApiError> {
//         let user_query = self.ship_model.to_lowercase();
//         let choices = match database::all_ship_models(pool).await {
//             Ok(x) => x
//                 .into_iter()
//                 .filter(|s| s.name.to_lowercase().contains(&user_query))
//                 .take(25)
//                 .collect::<Vec<_>>()
//                 .iter()
//                 .map(|s| CommandOptionChoice::String {
//                     name: s.name.to_string(),
//                     value: s.id.to_string(),
//                 })
//                 .collect::<Vec<_>>(),
//             Err(e) => {
//                 return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
//                     "Error querying database: {:?}",
//                     e
//                 )))
//             }
//         };

//         Ok(InteractionResponse::Autocomplete(Autocomplete { choices }))
//     }
// }

//TODO: Get rid of this when testing is done and a real database is in use
#[derive(Debug)]
#[allow(dead_code)]
pub struct Ship {
    pub model: Uuid,
    pub name: Option<String>,
}

pub static mut FAKEDB: Vec<Ship> = Vec::new();
