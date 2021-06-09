
use serde::{Deserialize, Serialize};

use crate::{snowflake::Snowflake, user::User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    //TODO https://discord.com/developers/docs/resources/channel#channel-object
    id: Snowflake,
    r#type: ChannelType,
    guild_id: Option<Snowflake>,
    position: Option<u32>,
    permission_overwrites: Option<Vec<Overwrite>>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<Snowflake>,
    bitrate: Option<u32>,
    user_limit: Option<u32>,
    rate_limit_per_user: Option<u32>,
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<Snowflake>,
    application_id: Option<Snowflake>,
    parent_id: Option<Snowflake>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialChannel {
    //TODO https://discord.com/developers/docs/resources/channel#channel-object
}
