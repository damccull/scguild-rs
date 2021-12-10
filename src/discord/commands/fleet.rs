use async_trait::async_trait;
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    command::{Command, CommandType},
    interaction::{ApplicationCommand, Interaction},
};
use twilight_util::builder::command::CommandBuilder;

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
        .build()
    }

    #[tracing::instrument(name = "Discord Interaction - FLEET")]
    async fn api_handler(_: Interaction) -> Result<InteractionResponse, DiscordApiError> {
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
