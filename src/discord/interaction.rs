use twilight_model::application::{callback::{CallbackData, InteractionResponse}, interaction::ApplicationCommand};
use twilight_util::builder::CallbackDataBuilder;

pub struct Response;
impl Response {
    pub fn ack() -> InteractionResponse {
        InteractionResponse::DeferredChannelMessageWithSource(CallbackDataBuilder::new().build())
    }

    pub fn message(message: impl Into<String>) -> InteractionResponse {
        let message = message.into();
        InteractionResponse::ChannelMessageWithSource(Self::_message(message))
    }

    fn _message(message: String) -> CallbackData {
        if message.is_empty() {
            panic!("empty message is disallowed");
        }
        CallbackDataBuilder::new().content(message).build()
    }
}

pub struct Interaction {
    pub command: ApplicationCommand,
}

impl Interaction {
    pub const fn new(command: ApplicationCommand) -> Self {
        Self { command }
    }

    /// Acknowledge the interaction, useful on commands that take a while to finish.
    /// 
    /// After calling thi, use [`Interaction::update_response`] to add the finished response.
    /// 
    /// <https://discord.com/developers/docs/interactions/slash-commands#interaction-response-object>
    pub async fn ack(&self) -> Result<(), twilight_http::Error> {
        todo!()
    }
}
