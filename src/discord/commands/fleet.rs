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
        .option(
            SubCommandGroupBuilder::new("hangar".into(), "Manage ships in your fleet".into())
                .subcommands([
                    SubCommandBuilder::new(
                        "list".into(),
                        "List all the ships in your (or someone else's) fleet privately.".into(),
                    ),
                    SubCommandBuilder::new(
                        "show".into(),
                        "Show the whole channel the ships in your fleet.".into(),
                    ),
                    SubCommandBuilder::new("add".into(), "Add a new ship to your fleet.".into())
                        .option(StringBuilder::new(
                            "name".into(),
                            "What you want your ship to be named.".into(),
                        ))
                        .option(StringBuilder::new(
                            "description".into(),
                            "Describe your ship".into(),
                        )),
                    SubCommandBuilder::new(
                        "remove".into(),
                        "Remove a ship from your fleet.".into(),
                    ),
                    SubCommandBuilder::new(
                        "rename".into(),
                        "Remove a ship from your fleet.".into(),
                    ),
                ]),
        )
        .option(
            SubCommandGroupBuilder::new("wishlist".into(), "Manage ships in your wishlist".into())
                .subcommands([
                    SubCommandBuilder::new(
                        "list".into(),
                        "List all the ships in your (or someone else's) wishlist privately.".into(),
                    ),
                    SubCommandBuilder::new(
                        "show".into(),
                        "Show the whole channel the ships in your wishlist.".into(),
                    ),
                    SubCommandBuilder::new("add".into(), "Add a new ship to your wishlist.".into()),
                    SubCommandBuilder::new(
                        "remove".into(),
                        "Remove a ship from your wishlist.".into(),
                    ),
                    SubCommandBuilder::new(
                        "rename".into(),
                        "Remove a ship from your wishlist.".into(),
                    ),
                ]),
        )
        .build()
    }

    #[tracing::instrument(name = "Discord Interaction - FLEET")]
    async fn api_handler(
        cmd: twilight_model::application::interaction::Interaction,
    ) -> Result<InteractionResponse, DiscordApiError> {
        let x = cmd;
        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackData {
                allowed_mentions: None,
                flags: None,
                tts: None,
                content: Some(
                    "This command will show fleet info and allow you to manage it.".to_string(),
                ),
                embeds: Default::default(),
                components: Default::default(),
            },
        ))
    }
}
