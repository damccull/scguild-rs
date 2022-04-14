use anyhow::{bail, Result};
use fnv::FnvHashMap;
use sqlx::PgPool;
use std::{convert::TryFrom, str::FromStr};
use twilight_model::{
    application::{
        command::{CommandOption, CommandOptionChoice},
        interaction::{
            application_command::{CommandDataOption, CommandOptionValue},
            application_command_autocomplete::ApplicationCommandAutocompleteDataOption,
            ApplicationCommand, ApplicationCommandAutocomplete,
        },
    },
    channel::message::MessageFlags,
    http::interaction::InteractionResponseData,
};
use twilight_util::builder::{
    command::{StringBuilder, SubCommandBuilder},
    InteractionResponseDataBuilder,
};
use uuid::Uuid;

use crate::{
    database,
    discord::{
        api::DiscordApiError, twilight_interactions_extensions::InteractionAutocompleteOption,
        DiscordSubcommand,
    },
};

use super::{Ship, FAKEDB};

#[derive(Clone, Debug)]
pub struct AddCommand {
    /// The model ID of ship you want to add.
    /// Note: This is a partial user-typed string during autocomplete
    /// and a UUID in String format for the final result.
    pub ship_model: InteractionAutocompleteOption<String>,
    /// The name of the ship. (Optional)
    pub ship_name: Option<String>,
}

// These constants are used to ensure matching strings are used in multiple places in the code
const OPTION_SHIP_MODEL_NAME: &str = "ship_model";
const OPTION_SHIP_MODEL_DESCRIPTION: &str = "type the ship model";
const OPTION_SHIP_NAME_NAME: &str = "ship_name";
const OPTION_SHIP_NAME_DESCRIPTION: &str = "type the name of the ship";

impl DiscordSubcommand for AddCommand {
    const NAME: &'static str = "add";
    const DESCRIPTION: &'static str = "Add a new ship to your fleet.";

    fn register() -> CommandOption {
        SubCommandBuilder::new(Self::NAME.into(), Self::DESCRIPTION.into())
            .option(
                StringBuilder::new(
                    OPTION_SHIP_MODEL_NAME.into(),
                    OPTION_SHIP_MODEL_DESCRIPTION.into(),
                )
                .required(true)
                .autocomplete(true),
            )
            .option(StringBuilder::new(
                OPTION_SHIP_NAME_NAME.into(),
                OPTION_SHIP_NAME_DESCRIPTION.into(),
            ))
            .build()
    }
}

impl AddCommand {
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

        let id = match Uuid::from_str(user_query.as_str()) {
            Ok(id) => id,
            Err(e) => {
                tracing::warn!("Unable to parse ID string as UUID: {:?}", e);
                return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                    "Unable to parse ID string as UUID: {:?}",
                    e
                )));
            }
        };

        // Non-autocomplete gets the 'value' of the choices, so here we expext a UUID of the
        // ship model's ID
        let ship_model = match database::get_ship_by_id(pool, id).await {
            Ok(x) => x,
            Err(e) => {
                tracing::warn!("Unable to find ship model in database: {:?}", e);
                return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                    "Unable to find ship model in database: {:?}",
                    e
                )));
            }
        };

        let ship_name = match self.ship_name.to_owned() {
            Some(name) => format!(" named _{}_", name),
            None => "".into(),
        };
        unsafe {
            FAKEDB.push(Ship {
                model: ship_model.id,
                name: self.ship_name.clone(),
            });
        }
        Ok(InteractionResponseDataBuilder::new()
            .content(format!(
                "Adding a {}{} to the fleet.",
                ship_model.name, ship_name
            ))
            .flags(MessageFlags::EPHEMERAL)
            .build())
    }

    #[tracing::instrument(
        name = "Discord Interaction - FLEET ADD AUTOCOMPLETE HANDLER",
        skip(self, pool)
    )]
    pub async fn autocomplete_handler(
        &self,
        autocomplete: &ApplicationCommandAutocomplete,
        pool: &PgPool,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        let user_query = match self.ship_model.clone() {
            InteractionAutocompleteOption::Partial(x) => x,
            _ => {
                return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                    "ship_model option seems to be incorrect"
                )))
            }
        };

        tracing::debug!("`user_query` is '{}'", user_query);

        let choices = match database::get_ships_by_model_name(pool, user_query).await {
            Ok(m) => m
                .iter()
                .take(25)
                .map(|s| CommandOptionChoice::String {
                    name: s.name.to_string(),
                    value: s.id.to_string(),
                })
                .collect::<Vec<_>>(),
            Err(e) => {
                tracing::warn!("Unable to parse given string as UUID: {:?}", e);
                return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                    "Unable to find ship model in database: {:?}",
                    &self.ship_model
                )));
            }
        };

        let response_data = InteractionResponseDataBuilder::new()
            .choices(choices)
            .build();

        Ok(response_data)
    }
}
impl TryFrom<Vec<CommandDataOption>> for AddCommand {
    type Error = anyhow::Error;

    fn try_from(options: Vec<CommandDataOption>) -> Result<Self> {
        if let CommandOptionValue::SubCommand(subcommand_options) = options[0].value.clone() {
            let mut map = FnvHashMap::default();
            subcommand_options.iter().enumerate().for_each(|(i, o)| {
                map.insert(o.name.clone(), i);
            });
            tracing::debug!("The MAP says: {:#?}", map);

            Ok(Self {
                ship_model: InteractionAutocompleteOption::Complete(
                    if map.contains_key(OPTION_SHIP_MODEL_NAME) {
                        tracing::debug!(
                            "THE SHIP_MODEL IS: {:#?}",
                            subcommand_options[map[OPTION_SHIP_MODEL_NAME]].value
                        );
                        if let CommandOptionValue::String(ship_model) = subcommand_options
                            [map[OPTION_SHIP_MODEL_NAME]]
                            .value
                            .clone()
                        {
                            ship_model
                        } else {
                            bail!(
                                "add command '{}' is not a string: {:#?}",
                                OPTION_SHIP_MODEL_NAME,
                                options
                            )
                        }
                    } else {
                        bail!("add command '{}' option missing", OPTION_SHIP_MODEL_NAME)
                    },
                ),
                ship_name: {
                    if map.contains_key(OPTION_SHIP_NAME_NAME) {
                        if let CommandOptionValue::String(ship_name) =
                            subcommand_options[map[OPTION_SHIP_NAME_NAME]].value.clone()
                        {
                            Some(ship_name)
                        } else {
                            bail!(
                                "add command '{}' is not a string: {:#?}",
                                OPTION_SHIP_NAME_NAME,
                                options
                            )
                        }
                    } else {
                        None
                    }
                },
            })
        } else {
            bail!("option '{}' is not a SubCommand", Self::NAME)
        }
    }
}
impl TryFrom<Vec<ApplicationCommandAutocompleteDataOption>> for AddCommand {
    type Error = anyhow::Error;

    fn try_from(options: Vec<ApplicationCommandAutocompleteDataOption>) -> Result<Self> {
        let mut map = FnvHashMap::default();

        options[0].options.iter().enumerate().for_each(|(i, o)| {
            map.insert(o.name.clone(), i);
        });

        Ok(Self {
            ship_model: {
                if map.contains_key(OPTION_SHIP_MODEL_NAME) {
                    InteractionAutocompleteOption::Partial(
                        options[0].options[map[OPTION_SHIP_MODEL_NAME]]
                            .value
                            .clone()
                            .ok_or_else(|| anyhow::anyhow!("No such ship model"))?,
                    )
                } else {
                    {
                        return Err(anyhow::anyhow!("Unable to get ship_model"));
                    }
                }
            },

            ship_name: {
                if map.contains_key(OPTION_SHIP_NAME_NAME) {
                    options[0].options[map[OPTION_SHIP_NAME_NAME]].value.clone()
                } else {
                    None
                }
            },
        })
    }
}
