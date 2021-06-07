use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

#[derive(Debug, Serialize, Deserialize)]
struct Interaction {
    /// The ID of the interaction.
    pub id: Snowflake,
    /// The ID of the application this interaction is for.
    pub application_id: Snowflake,
    /// The type of interaction. Discord InteractionType
    pub r#type: usize,
    /// The command data payload. Discord ApplicationCommandInteractionData
    pub data: Option<InteractionData>,
    /// The guild interaction was sent from.
    pub guild_id: Option<Snowflake>,
    /// The channel interaction was sent from.
    pub channel_id: Option<Snowflake>,
    /// Guild member data for the invoking user, including permissions. Discord guild member object.
    /// *Sent when the command is invoked from a guild.*
    pub member: Option<GuildMember>,
    /// User object for the invoking user, if invoked in a DM. Discord User object.
    /// *Sent when the command is invoked from a direct message.*
    pub user: Option<User>,
    /// A continuation token for responding to the interaction. String.
    pub token: String,
    /// A read-only property. Currently always `1`.
    pub version: u8,
    /// For components, the message they were attached to. Discord message object.
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GuildMember {
    pub user: User,
    pub roles: Vec<String>,
    pub premium_since: Option<String>,
    pub permissions: String,
    pub pending: bool,
    pub nick: Option<String>,
    pub mute: bool,
    pub joined_at: String,
    pub is_pending: bool,
    pub deaf: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    pub id: u64,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct InteractionData {
    pub options: Vec<NameValuePair<String, String>>,
    pub name: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NameValuePair<T, U> {
    pub name: T,
    pub value: U,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
