use serde::{Deserialize, Serialize};

use crate::snowflake::{self, Snowflake};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChannelMention {
    /// ID of the channel
    id: Snowflake,
    /// ID of the guild containing the channel
    guild_id: Snowflake,
    /// The type of channel
    r#type: ChannelType,
    /// The name of the channel
    name: String,
}
