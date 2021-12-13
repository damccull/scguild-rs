use async_trait::async_trait;
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    command::{Command, CommandType},
    interaction::{application_command::{CommandOptionValue, CommandDataOption}, ApplicationCommand},
};
use twilight_util::builder::command::{CommandBuilder, StringBuilder, SubCommandBuilder};

use crate::discord::{api::DiscordApiError, SlashCommand};

pub struct Fleet(pub ApplicationCommand);

impl Fleet {
    pub const CMD_LIST_NAME: &'static str = "list";
    pub const CMD_LIST_DESC: &'static str =
        "List all the ships in your (or someone else's) fleet privately.";
    pub const CMD_SHOW_NAME: &'static str = "show";
    pub const CMD_SHOW_DESC: &'static str = "Show the whole channel the ships in your fleet.";
    pub const CMD_ADD_NAME: &'static str = "add";
    pub const CMD_ADD_DESC: &'static str = "Add a new ship to your fleet.";
    pub const CMD_REMOVE_NAME: &'static str = "remove";
    pub const CMD_REMOVE_DESC: &'static str = "Remove a ship from your fleet.";
    pub const CMD_RENAME_NAME: &'static str = "rename";
    pub const CMD_RENAME_DESC: &'static str = "Rename a ship in your fleet.";

    fn cmd_add_handler(cmd: &CommandDataOption) -> Result<InteractionResponse, DiscordApiError> {
        let x = cmd;
        dbg!(&x);
        let ship_name = &x.name;

        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackData {
                allowed_mentions: None,
                flags: None,
                tts: None,
                content: Some(format!("Adding a ship named '{}' to the fleet.", ship_name)),
                embeds: Default::default(),
                components: Default::default(),
            },
        ))
    }

    fn cmd_remove_handler(
        cmd: &ApplicationCommand,
    ) -> Result<InteractionResponse, DiscordApiError> {
        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackData {
                allowed_mentions: None,
                flags: None,
                tts: None,
                content: Some("Removing a ship from the fleet.".into()),
                embeds: Default::default(),
                components: Default::default(),
            },
        ))
    }
}

#[async_trait]
impl SlashCommand for Fleet {
    const NAME: &'static str = "fleet";

    fn define() -> Command {
        CommandBuilder::new(
            Fleet::NAME.into(),
            "Displays and manages a player's fleet.".into(),
            CommandType::ChatInput,
        )
        .option(SubCommandBuilder::new(
            Fleet::CMD_LIST_NAME.into(),
            Fleet::CMD_LIST_DESC.into(),
        ))
        .option(SubCommandBuilder::new(
            Fleet::CMD_SHOW_NAME.into(),
            Fleet::CMD_SHOW_DESC.into(),
        ))
        .option(
            SubCommandBuilder::new(Fleet::CMD_ADD_NAME.into(), Fleet::CMD_ADD_DESC.into())
                .option(
                    StringBuilder::new(
                        "name".into(),
                        "What you want your ship to be named.".into(),
                    )
                    .required(true),
                )
                .option(StringBuilder::new(
                    "description".into(),
                    "Describe your ship".into(),
                )),
        )
        .option(SubCommandBuilder::new(
            Fleet::CMD_REMOVE_NAME.into(),
            Fleet::CMD_REMOVE_DESC.into(),
        ))
        .option(SubCommandBuilder::new(
            Fleet::CMD_RENAME_NAME.into(),
            Fleet::CMD_RENAME_DESC.into(),
        ))
        .build()
    }

    #[tracing::instrument(name = "Discord Interaction - FLEET")]
    async fn api_handler(cmd: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError> {
        match cmd.data.options.get(0) {
            Some(subcommand) => match subcommand.name.as_str() {
                Fleet::CMD_ADD_NAME => Fleet::cmd_add_handler(cmd.data.options.get(0).unwrap()),
                Fleet::CMD_REMOVE_NAME => Fleet::cmd_remove_handler(cmd),
                _ => Err(DiscordApiError::UnsupportedCommand(
                    cmd.data.name.to_owned(),
                )),
            },
            None => {
                return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                    "Command data was empty."
                )));
            }
        }
    }
}
