use async_trait::async_trait;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser, CommandInputData};
use twilight_model::application::{interaction::ApplicationCommand, callback::{InteractionResponse, CallbackData}};

use crate::discord::{api::DiscordApiError, SlashCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello to other members")]
pub struct HelloCommand {
    /// Message to send
    pub message: String,
    /// User to send the message to
    pub user: Option<ResolvedUser>,
}
#[async_trait]
impl SlashCommand for HelloCommand {
    const NAME: &'static str = "hello";

    fn define() -> twilight_model::application::command::Command {
        todo!()
    }

    async fn api_handler(cmd: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError> {
        {
            let x: CommandInputData = cmd.data.clone().into();
            let x: HelloCommand = HelloCommand::from_interaction(x).unwrap();
            let nick = match x.user {
                Some(ref y) => match y.member.clone() {
                    Some(z) => z.nick.unwrap_or_else(|| y.resolved.name.to_owned()),
                    None => y.resolved.name.to_owned(),
                },
                None => "everyone".into(),
            };
            let message = x.message;
            Ok(InteractionResponse::ChannelMessageWithSource(
                CallbackData {
                    allowed_mentions: None,
                    flags: None,
                    tts: None,
                    content: Some(format!("{}, {}", message, nick)),
                    embeds: Default::default(),
                    components: Default::default(),
                },
            ))
        }
    }
}
