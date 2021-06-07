use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
struct Interaction {
    /// The ID of the interaction.
    id: Snowflake,
    /// The ID of the application this interaction is for.
    application_id: Snowflake,
    /// The type of interaction. Discord InteractionType
    r#type: usize,
    /// The command data payload. Discord ApplicationCommandInteractionData
    data: Option<InteractionData>,
    /// The guild interaction was sent from.
    guild_id: Option<Snowflake>,
    /// The channel interaction was sent from.
    channel_id: Option<Snowflake>,
    /// Guild member data for the invoking user, including permissions. Discord guild member object.
    /// *Sent when the command is invoked from a guild.*
    member: Option<GuildMember>,
    /// User object for the invoking user, if invoked in a DM. Discord User object.
    /// *Sent when the command is invoked from a direct message.*
    user: Option<User>,
    /// A continuation token for responding to the interaction. String.
    token: String,
    /// A read-only property. Currently always `1`.
    version: u8,
    /// For components, the message they were attached to. Discord message object.
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GuildMember {
    user: User,
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
struct User {
    id: u64,
    username: String,
    avatar: String,
    discriminator: String,
    public_flags: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct InteractionData {
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
