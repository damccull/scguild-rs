use std::str::FromStr;

use crate::{
    database,
    discord::{api::DiscordApiError, format_simple_message_response},
};
use sqlx::PgPool;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand, ResolvedUser};
use twilight_model::{application::{
    command::CommandOptionChoice,
    interaction::ApplicationCommand,
}, http::interaction::InteractionResponseData};
use uuid::Uuid;

#[allow(clippy::large_enum_variant)]
#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "fleet", desc = "Manage or view your fleet, or show it off.")]
pub enum FleetCommand {
    #[command(name = "add")]
    Add(AddCommand),
    #[command(name = "list")]
    List(ListCommand),
    #[command(name = "remove")]
    Remove(RemoveCommand),
    #[command(name = "rename")]
    Rename(RenameCommand),
    #[command(name = "show")]
    Show(ShowCommand),
}
impl FleetCommand {
    pub const NAME: &'static str = "fleet";
}

impl FleetCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET", skip(cmd, pool))]
    pub async fn handler(
        cmd: &ApplicationCommand,
        pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        let x: CommandInputData = cmd.data.clone().into();
        match FleetCommand::from_interaction(x) {
            Ok(subcommand) => match subcommand {
                FleetCommand::Add(add_command) => add_command.handler(cmd, pool).await,
                FleetCommand::List(list_command) => list_command.handler(cmd, pool).await,
                FleetCommand::Remove(_) => todo!(),
                FleetCommand::Rename(_) => todo!(),
                FleetCommand::Show(show_command) => show_command.handler(cmd, pool).await,
            },
            Err(e) => {
                return Err(DiscordApiError::UnsupportedCommand(format!(
                    "Something went wrong parsing the interaction: {}",
                    e
                )));
            }
        }
    }

    #[tracing::instrument(name = "Discord Interaction - FLEET ADD AUTOCOMPLETE", skip(cmd, pool))]
    pub async fn autocomplete_handler(
        cmd: &ApplicationCommand,
        pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        let x: CommandInputData = cmd.data.clone().into();
        match FleetCommandPartial::from_interaction(x) {
            Ok(subcommand) => match subcommand {
                //FleetCommandPartial::Add(add_command) => add_command.handle(cmd, pool).await,
                _ => return Err(DiscordApiError::AutocompleteUnsupported),
            },
            Err(e) => {
                return Err(DiscordApiError::UnsupportedCommand(format!(
                    "Something went wrong parsing the interaction: {}",
                    e
                )));
            }
        }
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "add", desc = "Add a ship to your fleet.")]
pub struct AddCommand {
    /// The model of ship you want to add.
    #[command(rename = "model", autocomplete = true)]
    pub ship_model: String,
    /// The name of the ship. (Optional)
    #[command(rename = "name")]
    pub ship_name: Option<String>,
}

impl AddCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET ADD", skip(self, pool))]
    async fn handler(
        &self,
        cmd: &ApplicationCommand,
        pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        let ship_id = match Uuid::from_str(&self.ship_model.to_owned()) {
            Ok(x) => x,
            Err(e) => {
                tracing::warn!("Unable to parse given string as UUID: {:?}", e);
                return Ok(format_simple_message_response(&format!(
                    "Unable to find ship model in database: {}",
                    &self.ship_model
                )));
            }
        };
        match database::get_ship_by_id(pool, ship_id).await {
            Ok(model) => {
                let ship_name = match self.ship_name.to_owned() {
                    Some(name) => format!(" named _{}_", name),
                    None => "".into(),
                };
                unsafe {
                    FAKEDB.push(Ship {
                        model: model.id.to_owned(),
                        name: self.ship_name.clone(),
                    });
                }
                Ok(format_simple_message_response(&format!(
                    "Adding a {}{} to the fleet.",
                    model.name, ship_name
                )))
            }
            Err(e) => match e {
                database::DatabaseError::RecordNotFoundError(_) => {
                    tracing::warn!(
                        "Unable to find ship model in database: {}",
                        &self.ship_model
                    );
                    Ok(format_simple_message_response(&format!(
                        "Unable to find ship model in database: {}",
                        &self.ship_model
                    )))
                }
                database::DatabaseError::UnexpectedError(_) => {
                    return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                        "An unexpected error occurred looking for the ship model.",
                    )))
                }
            },
        }
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(
    name = "list",
    desc = "Privately list the ships in your, or the specified user's, fleet."
)]
pub struct ListCommand {
    /// The user who's fleet you'd like to see. (Optional)
    pub user: Option<ResolvedUser>,
}

impl ListCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET LIST", skip(_pool))]
    async fn handler(
        &self,
        _cmd: &ApplicationCommand,
        _pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        Ok(format_simple_message_response(
            "Privately perusing the fleet.",
        ))
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "remove", desc = "Remove a ship from your fleet.")]
pub struct RemoveCommand {}

impl RemoveCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET REMOVE", skip(_pool))]
    async fn handler(
        _cmd: &ApplicationCommand,
        _pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        Ok(format_simple_message_response(
            "Removing a ship from the fleet.",
        ))
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "rename", desc = "Remame a ship in your fleet.")]
pub struct RenameCommand {}

impl RenameCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET RENAME", skip(_pool))]
    async fn handler(
        _cmd: &ApplicationCommand,
        _pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        Ok(format_simple_message_response(
            "Renaming a ship in the fleet.",
        ))
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "show", desc = "Show your fleet to the channel.")]
pub struct ShowCommand;

impl ShowCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET SHOW", skip(_pool))]
    async fn handler(
        &self,
        _cmd: &ApplicationCommand,
        _pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        unsafe {
            Ok(format_simple_message_response(&format!(
                "Showing off the fleet.\n```\n{:?}\n```",
                FAKEDB
            )))
        }
    }
}

// AUTOCOMPLETE command models

#[derive(CommandModel, Debug)]
#[command(partial = true)]
pub enum FleetCommandPartial {
    #[command(name = "add")]
    Add(AddCommandPartial),
}

#[derive(CommandModel, Debug)]
#[command(partial = true)]
pub struct AddCommandPartial {
    /// The model of ship you want to add.
    #[command(rename = "model")]
    pub ship_model: String,
}

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
struct Ship {
    pub model: Uuid,
    pub name: Option<String>,
}

static mut FAKEDB: Vec<Ship> = Vec::new();
