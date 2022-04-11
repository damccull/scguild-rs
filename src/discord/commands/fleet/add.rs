use anyhow::{bail, Context, Result};
use sqlx::PgPool;
use std::convert::TryFrom;
use twilight_model::{
    application::{
        command::CommandOption,
        interaction::{
            application_command::{CommandDataOption, CommandOptionValue},
            application_command_autocomplete::ApplicationCommandAutocompleteDataOption,
            ApplicationCommand, ApplicationCommandAutocomplete,
        },
    },
    http::interaction::InteractionResponseData,
};
use twilight_util::builder::command::SubCommandBuilder;

use crate::{
    database,
    discord::{
        api::DiscordApiError, format_simple_message_response,
        twilight_interactions_extensions::InteractionAutocompleteOption,
    },
};

use super::{Ship, FAKEDB};

#[derive(Clone, Debug)]
pub struct AddCommand {
    /// The model of ship you want to add.
    pub ship_model: InteractionAutocompleteOption<String>,
    /// The name of the ship. (Optional)
    pub ship_name: Option<String>,
}

impl AddCommand {
    pub const NAME: &'static str = "add";
    pub const DESCRIPTION: &'static str = "Add a new ship to your fleet.";

    pub fn register() -> CommandOption {
        SubCommandBuilder::new(Self::NAME.into(), Self::DESCRIPTION.into()).build()
    }

    #[tracing::instrument(name = "Discord Interaction - FLEET ADD HANDLER", skip(self, pool))]
    pub async fn handler(
        &self,
        cmd: &ApplicationCommand,
        pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        let user_query = match self.ship_model.clone() {
            InteractionAutocompleteOption::Complete(x) => x,
            _ => {
                return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                    "ship_model seems to be incorrect"
                )))
            }
        };

        let result: Result<InteractionResponseData, DiscordApiError> =
            match database::get_ships_by_model_name(pool, user_query.clone()).await {
                Ok(mut x) if x.len() == 1 => {
                    if let Some(m) = x.pop() {
                        let ship_name = match self.ship_name.to_owned() {
                            Some(name) => format!(" named _{}_", name),
                            None => "".into(),
                        };
                        unsafe {
                            FAKEDB.push(Ship {
                                model: m.id.to_owned(),
                                name: self.ship_name.clone(),
                            });
                        }
                        Ok(format_simple_message_response(&format!(
                            "Adding a {}{} to the fleet.",
                            m.name, ship_name
                        )))
                    } else {
                        tracing::warn!("Weird corrupt ShipModel case: {:?}", user_query);
                        return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                            "Weird corrupt ShipModel case: {:?}",
                            &self.ship_model
                        )));
                    }
                }
                Ok(x) if x.is_empty() => {
                    tracing::warn!("No matches found where one expected: {:?}", user_query);
                    return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                        "No matches found where one expected: {:?}",
                        &self.ship_model
                    )));
                }
                Ok(_x) => {
                    tracing::warn!(
                        "Multiple matches found where only one expected: {:?}",
                        user_query
                    );
                    return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                        "Multiple matches found where only one expected: {:?}",
                        &self.ship_model
                    )));
                }
                Err(e) => {
                    tracing::warn!("Unable to parse given string as UUID: {:?}", e);
                    return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                        "Unable to find ship model in database: {:?}",
                        &self.ship_model
                    )));
                }
            };

        result
    }

    #[tracing::instrument(
        name = "Discord Interaction - FLEET ADD AUTOCOMPLETE HANDLER",
        skip(self, _pool)
    )]
    fn autocomplete_handler(
        &self,
        autocomplete: &ApplicationCommandAutocomplete,
        _pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        todo!()
    }
}
impl TryFrom<Vec<CommandDataOption>> for AddCommand {
    type Error = anyhow::Error;

    fn try_from(options: Vec<CommandDataOption>) -> Result<Self> {
        //TODO: Replace the finds with a fnv hashmap for increased performance

        //Create an index into the vec by name
        //let valuemap = options.iter().

        Ok(Self {
            ship_model: InteractionAutocompleteOption::Complete(
                if let CommandOptionValue::String(ship_model) = &options
                    .iter()
                    .find(|option| option.name == "ship_model")
                    .context("add command missing 'ship_model' option")?
                    .value
                {
                    ship_model.to_string()
                } else {
                    bail!("add command 'ship_model' is not a string: {options:#?}")
                },
            ),
            ship_name: if let CommandOptionValue::String(ship_name) = &options
                .iter()
                .find(|option| option.name == "ship_name")
                .context("add command missing 'ship_name' option")?
                .value
            {
                Some(ship_name.into())
            } else {
                bail!("add command 'ship_name' is not a string: {options:#?}")
            },
        })
    }
}
impl From<Vec<ApplicationCommandAutocompleteDataOption>> for AddCommand {
    fn from(options: Vec<ApplicationCommandAutocompleteDataOption>) -> Self {
        Self {
            ship_model: InteractionAutocompleteOption::Partial(
                options
                    .iter()
                    .find(|option| option.name == "ship_model")
                    .map_or("".to_string(), |o| o.value.clone().unwrap_or_default()),
            ),
            ship_name: options
                .iter()
                .find(|option| option.name == "ship_name")
                .and_then(|o| o.value.clone()),
        }
    }
}
