use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    command::{Command, CommandType},
    interaction::{ApplicationCommand, Interaction},
};
use twilight_util::builder::command::CommandBuilder;

use crate::discord::{api::DiscordApiError, SlashCommand};

pub struct About(pub ApplicationCommand);
impl About {
    #[tracing::instrument(name = "Discord Interaction - ABOUT")]
    pub async fn about(interaction: Interaction) -> Result<InteractionResponse, DiscordApiError> {
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

    // fn run< 'async_trait>(self)-> core::pin::Pin<Box<dyncore::future::Future<Output=anyhow::Result<()> > + core::marker::Send+ 'async_trait> >whereSelf: 'async_trait {
    //     let ctx = Interaction::new(self.0);
    // }
}
