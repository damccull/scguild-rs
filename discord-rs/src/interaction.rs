use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    application_command::ApplicationCommandOptionType, message_components::MessageComponentType,
    snowflake::Snowflake,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Interaction {
    /// The ID of the interaction.
    pub id: Snowflake,
    /// The ID of the application this interaction is for.
    pub application_id: Snowflake,
    /// The type of interaction. Discord InteractionType
    pub r#type: InteractionType,
    /// The command data payload. Discord ApplicationCommandInteractionData
    pub data: Option<ApplicationCommandInteractionData>,
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildMember {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionData {
    /// The ID of the invoked command
    pub id: Snowflake,
    /// The name of the invoked command
    pub name: String,
    /// Converted users + roles + channels
    pub resolved: Option<ApplicationCommandInteractionDataResolved>,
    /// The parameters + values from the user
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    /// For components, the custom_id of the component
    pub custom_id: String,
    /// For components, the type of the component
    pub component_type: MessageComponentType,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionDataResolved {
    pub users: HashMap<Snowflake, User>,
    pub members: Option<HashMap<Snowflake, PartialGuildMember>>,
    pub roles: Option<HashMap<Snowflake, Role>>,
    pub channels: Option<HashMap<Snowflake, PartialChannel>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialGuildMember {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Role {}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialChannel {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApplicationCommandInteractionDataOption {
    /// The name of the parameter
    name: String,
    /// The type of Application Command Option this is
    r#type: ApplicationCommandOptionType,
    /// The value of the pair
    value: Option<String>,
    /// Present if this option is a group or subcommand
    options: Option<Box<Vec<ApplicationCommandInteractionDataOption>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NameValuePair<T, U> {
    pub name: T,
    pub value: U,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
