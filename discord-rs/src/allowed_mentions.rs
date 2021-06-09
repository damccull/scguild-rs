use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AllowedMentions {
    /// An array of allowed mention types to partse from the content.
    parse: Vec<String>,
    /// An array of role_ids to mention (Max size of 100).
    roles: Vec<Snowflake>,
    /// An array of user_ids to mention (Max size of 100).
    users: Vec<Snowflake>,
    /// For replies, whether to mention the author of the message being replied to (default false).
    replied_user: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructName {}

mod allowed_mention_types {
    const ROLE_MENTIONS: &str = "roles";
    const USER_MENTION: &str = "users";
    const EVERYONE_MENTIONS: &str = "everyone";
}
