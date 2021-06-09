//TODO https://discord.com/developers/docs/interactions/slash-commands#interaction-response
use serde::{Deserialize, Serialize};

use crate::embed::Embed;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractionResponse {
    /// The type of response.
    r#type: InteractionCallbackType,
    /// An optional response message.
    data: Option<InteractionApplicationCommandCallbackData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum InteractionCallbackType {
    /// Used to ACK a ping.
    Pong = 1,
    /// Respond to an interaction with a message.
    ChannelMessageWithSource = 4,
    /// ACK an interaction and edit a response later. User sees a loading state.
    DeferredChannelMessageWithSource = 5,
    /// For components, ACK an interaction and edit the original message later. The user does not see a loading state.
    DeferredUpdateMessage = 6,
    /// For components, edit the message the component was attached to.
    UpdateMessage = 7,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractionApplicationCommandCallbackData {
    tts: Option<bool>,
    content: Option<String>,    
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<AllowedMentions>,
    flags: Option<usize>, // TODO: Change this to a dedicated flags struct or enum or bitmask
    components: Option<MessageComponent>,
}
