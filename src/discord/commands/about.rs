use async_trait::async_trait;
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    command::{Command, CommandType},
    interaction::{ApplicationCommand, Interaction},
};
use twilight_util::builder::command::CommandBuilder;

use crate::discord::{api::DiscordApiError, SlashCommand};

pub struct About(pub ApplicationCommand);

#[async_trait]
impl SlashCommand for About {
    const NAME: &'static str = "about";

    fn define() -> Command {
        CommandBuilder::new(
            About::NAME.into(),
            "Get information about the application".into(),
            CommandType::ChatInput,
        )
        .build()
    }

    #[tracing::instrument(name = "Discord Interaction - ABOUT")]
    async fn api_handler(_: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError> {
        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackData {
                allowed_mentions: None,
                flags: None,
                tts: None,
                content: Some("Norseline Discord Bot v1. It is probably still lame.".to_string()),
                embeds: Default::default(),
                components: Default::default(),
            },
        ))
    }
}
