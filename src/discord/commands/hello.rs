use async_trait::async_trait;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    interaction::ApplicationCommand,
};

use crate::discord::{api::DiscordApiError, SlashCommand};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "wave", desc = "wave at other members")]
pub struct WaveSub {
    /// User to send the message to
    pub user: Option<ResolvedUser>,
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "speak", desc = "Say hello to other members")]
pub struct SpeakSub {
    /// Message to send
    pub message: String,
    /// User to send the message to
    pub user: Option<ResolvedUser>,
}

#[derive(CreateCommand, CommandModel, Debug)]
#[command(name = "hello", desc = "Greet other members")]
pub enum HelloCommand {
    #[command(name = "speak")]
    Speak(SpeakSub),
    #[command(name = "wave")]
    Wave(WaveSub),
}
impl HelloCommand {
    pub const NAME: &'static str = "hello";
}

#[async_trait]
impl SlashCommand for HelloCommand {
    #[tracing::instrument(name = "Discord Interaction - HELLO")]
    async fn handler(cmd: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError> {
        {
            let x: CommandInputData = cmd.data.clone().into();
            match HelloCommand::from_interaction(x) {
                Ok(x) => {
                    let message = match x {
                        HelloCommand::Speak(data) => {
                            let nick = match data.user {
                                Some(ref y) => match y.member.clone() {
                                    Some(z) => z.nick.unwrap_or_else(|| y.resolved.name.to_owned()),
                                    None => y.resolved.name.to_owned(),
                                },
                                None => "everyone".into(),
                            };
                            format!("{}, {}", data.message, nick)
                        }
                        HelloCommand::Wave(data) => {
                            let target_nick = match data.user {
                                Some(ref y) => {
                                    let name = match y.member.clone() {
                                        Some(z) => {
                                            z.nick.unwrap_or_else(|| y.resolved.name.to_owned())
                                        }
                                        None => y.resolved.name.to_owned(),
                                    };
                                    format!(", {}", name)
                                }
                                None => "".into(),
                            };
                            let sender_nick = match cmd.member.clone() {
                                Some(member) => match member.nick {
                                    Some(nick) => nick,
                                    None => match member.user {
                                        Some(user) => user.name,
                                        None => {
                                            return Err(DiscordApiError::UnexpectedError(
                                                anyhow::anyhow!(
                                                    "No nickname or user exists on interaction."
                                                ),
                                            ));
                                        }
                                    },
                                },
                                None => match cmd.user.clone() {
                                    Some(user) => user.name,
                                    None => {
                                        return Err(DiscordApiError::UnexpectedError(
                                            anyhow::anyhow!(
                                                "No member or user exists on interaction."
                                            ),
                                        ));
                                    }
                                },
                            };
                            format!("{} waves{}.", sender_nick, target_nick)
                        }
                    };
                    Ok(InteractionResponse::ChannelMessageWithSource(
                        CallbackData {
                            allowed_mentions: None,
                            flags: None,
                            tts: None,
                            content: Some(message),
                            embeds: Default::default(),
                            components: Default::default(),
                        },
                    ))
                }
                Err(e) => {
                    return Err(DiscordApiError::UnsupportedCommand(format!(
                        "Something went wrong parsing the interaction: {}",
                        e
                    )));
                }
            }
        }
    }

    #[tracing::instrument(name = "Discord Interaction - HELLO AUTOCOMPLETE")]
    async fn autocomplete_handler(
        cmd: &ApplicationCommand,
    ) -> Result<InteractionResponse, DiscordApiError> {
        Err(DiscordApiError::AutocompleteUnsupported)
    }
}
