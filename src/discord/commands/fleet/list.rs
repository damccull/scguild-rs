use anyhow::{bail, Result};
use fnv::FnvHashMap;
use std::convert::TryFrom;

use twilight_model::{
    application::{
        command::CommandOption,
        interaction::{
            application_command::{CommandDataOption, CommandOptionValue},
            ApplicationCommand,
        },
    },
    channel::message::MessageFlags,
    http::interaction::InteractionResponseData,
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::{
    command::{BooleanBuilder, SubCommandBuilder, UserBuilder},
    InteractionResponseDataBuilder,
};

use crate::discord::{api::DiscordApiError, DiscordSubcommand};

#[derive(Clone, Debug)]
pub struct ListCommand {
    user: Option<Id<UserMarker>>,
    show_everyone: bool,
}

// These constants are used to ensure matching strings are used in multiple places in the code
const OPTION_USER_NAME: &str = "user";
const OPTION_USER_DESCRIPTION: &str = "type the name of the user whose ships you want to see";
const OPTION_SHOW_EVERYONE_NAME: &str = "show_everyone";
const OPTION_SHOW_EVERYONE_DESCRIPTION: &str =
    "'true' if the list should be broadcast to the entire channel. Defaults to 'false'.";

impl DiscordSubcommand for ListCommand {
    const NAME: &'static str = "list";
    const DESCRIPTION: &'static str = "Displays all the ships in your fleet to just yourself.";

    fn register() -> CommandOption {
        SubCommandBuilder::new(Self::NAME.into(), Self::DESCRIPTION.into())
            .option(UserBuilder::new(
                OPTION_USER_NAME.into(),
                OPTION_USER_DESCRIPTION.into(),
            ))
            .option(BooleanBuilder::new(
                OPTION_SHOW_EVERYONE_NAME.into(),
                OPTION_SHOW_EVERYONE_DESCRIPTION.into(),
            ))
            .build()
    }
}
impl ListCommand {
    pub async fn handler(
        &self,
        cmd: &ApplicationCommand,
    ) -> Result<InteractionResponseData, DiscordApiError> {
        //First, see if a user was supplied
        if self.user.is_none() {
            //Return the user's own fleet
            //Ensure this isn't called with an empty user
            match &cmd.member {
                Some(u) => {
                    let x = match u.nick.clone() {
                        Some(m) => m,
                        None => match u.user.clone() {
                            Some(m) => m.name,
                            None => {
                                return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                                    "Unable to get nickname or username for guild member"
                                )));
                            }
                        },
                    };
                    let mut response =
                        InteractionResponseDataBuilder::new().content(format!("Fleet for {}", x));
                    if !self.show_everyone {
                        response = response.flags(MessageFlags::EPHEMERAL);
                    }
                    Ok(response.build())
                }
                _ => Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                    "Calling user is empty."
                ))),
            }
        } else {
            //Return the specified user's fleet
            //First ensure that the user ID was not empty
            let user_id = match self.user {
                Some(u) => u,
                None => {
                    return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                        "No user sent in the fleet list command"
                    )));
                }
            };
            let mut response = InteractionResponseDataBuilder::new()
                .content(format!("Showing ships for user ID: {}", user_id.to_string()));
            if !self.show_everyone {
                response = response.flags(MessageFlags::EPHEMERAL);
            }
            Ok(response.build())
        }
    }
}

impl TryFrom<Vec<CommandDataOption>> for ListCommand {
    type Error = anyhow::Error;

    fn try_from(options: Vec<CommandDataOption>) -> Result<Self> {
        if let CommandOptionValue::SubCommand(subcommand_options) = options[0].value.clone() {
            let mut map = FnvHashMap::default();
            subcommand_options.iter().enumerate().for_each(|(i, o)| {
                map.insert(o.name.clone(), i);
            });
            tracing::debug!("The MAP says: {:#?}", map);

            Ok(Self {
                user: {
                    if map.contains_key(OPTION_USER_NAME) {
                        if let CommandOptionValue::User(user) =
                            subcommand_options[map[OPTION_USER_NAME]].value.clone()
                        {
                            Some(user)
                        } else {
                            bail!("Can't find target user.")
                        }
                    } else {
                        None
                    }
                },
                show_everyone: {
                    if map.contains_key(OPTION_SHOW_EVERYONE_NAME) {
                        if let CommandOptionValue::Boolean(show_everyone) = subcommand_options
                            [map[OPTION_SHOW_EVERYONE_NAME]]
                            .value
                            .clone()
                        {
                            show_everyone
                        } else {
                            tracing::warn!(
                                "the {} option was not a boolean; defaulting to false",
                                OPTION_SHOW_EVERYONE_NAME.to_string()
                            );
                            false
                        }
                    } else {
                        false
                    }
                },
            })
        } else {
            bail!("option '{}' is not a SubCommand", Self::NAME)
        }
    }
}
