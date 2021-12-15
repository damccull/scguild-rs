use async_trait::async_trait;
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    command::{Command, CommandType},
    interaction::ApplicationCommand,
};
use twilight_util::builder::command::{CommandBuilder, SubCommandBuilder};

use crate::discord::{api::DiscordApiError, SlashCommand};

pub struct Wishlist(pub ApplicationCommand);

#[async_trait]
impl SlashCommand for Wishlist {
    const NAME: &'static str = "wishlist";

    #[tracing::instrument(name = "Discord Interaction - WISHLIST")]
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
