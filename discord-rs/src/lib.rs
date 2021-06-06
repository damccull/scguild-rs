use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct DiscordInteraction {
    /// The ID of the interaction. Discord Snowflake.
    id: String,
    /// The ID of the application this interaction is for. Discord Snowflake.
    application_id: String,
    /// The type of interaction. Discord InteractionType
    #[serde(rename = "type")]
    interaction_type: usize,
    /// The command data payload. Discord ApplicationCommandInteractionData
    data: Option<DiscordInteractionData>,
    /// The guild interaction was sent from. Discord Snowflake.
    guild_id: Option<String>,
    /// The channel interaction was sent from. Discord Snowflake.
    channel_id: Option<String>,
    /// Guild member data for the invoking user, including permissions. Discord guild member object.
    /// *Sent when the command is invoked from a guild.*
    member: Option<DiscordGuildMember>,
    /// User object for the invoking user, if invoked in a DM. Discord User object.
    /// *Sent when the command is invoked from a direct message.*
    user: Option<DiscordUser>,
    /// A continuation token for responding to the interaction. String.
    token: String,
    /// A read-only property. Currently always `1`.
    version: u8,
    /// For components, the message they were attached to. Discord message object.
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiscordGuildMember {
    user: DiscordUser,
    roles: Vec<String>,
    premium_since: Option<String>,
    permissions: String,
    pending: bool,
    nick: Option<String>,
    mute: bool,
    joined_at: String,
    is_pending: bool,
    deaf: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiscordUser {
    id: u64,
    username: String,
    avatar: String,
    discriminator: String,
    public_flags: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct DiscordInteractionData {
    options: Vec<NameValuePair<String, String>>,
    name: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NameValuePair<T, U> {
    name: T,
    value: U,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
