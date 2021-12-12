use async_trait::async_trait;
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    command::{Command, CommandType},
    interaction::ApplicationCommand,
};
use twilight_util::builder::command::{
    CommandBuilder, StringBuilder, SubCommandBuilder, SubCommandGroupBuilder,
};

use crate::discord::{api::DiscordApiError, SlashCommand};

pub struct Fleet(pub ApplicationCommand);

#[async_trait]
impl SlashCommand for Fleet {
    const NAME: &'static str = "fleet";

    fn define() -> Command {
        CommandBuilder::new(
            Fleet::NAME.into(),
            "Displays and manages a player's fleet.".into(),
            CommandType::ChatInput,
        )
        .option(SubCommandBuilder::new("test".into(), "test".into()))
        .option(SubCommandBuilder::new(
            "list".into(),
            "List all the ships in your (or someone else's) fleet privately.".into(),
        ))
        .option(SubCommandBuilder::new(
            "show".into(),
            "Show the whole channel the ships in your fleet.".into(),
        ))
        .option(
            SubCommandBuilder::new("add".into(), "Add a new ship to your fleet.".into())
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
            "remove".into(),
            "Remove a ship from your fleet.".into(),
        ))
        .option(SubCommandBuilder::new(
            "rename".into(),
            "Remove a ship from your fleet.".into(),
        ))
        .build()
    }

    #[tracing::instrument(name = "Discord Interaction - FLEET")]
    async fn api_handler(cmd: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError> {
        let result: String = match cmd.data.options.get(0) {
            Some(subcommand) => match subcommand.name.as_str() {
                "add" => "You would have just added a ship.".into(),
                "remove" => "you would have just removed a ship.".into(),
                _ => "No such command.".into(),
            },
            None => {
                return Err(DiscordApiError::UnexpectedError(anyhow::anyhow!(
                    "Command data was empty."
                )));
            }
        };

        dbg!(&cmd.data, &cmd.data.options.get(0).unwrap().name.as_str());
        //TODO: Figure out how to match against a subcommand or user input
        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackData {
                allowed_mentions: None,
                flags: None,
                tts: None,
                content: Some(result),
                embeds: Default::default(),
                components: Default::default(),
            },
        ))
    }
}
