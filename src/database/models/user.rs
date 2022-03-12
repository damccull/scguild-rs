use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    //pub discord_id: Option<DiscordUserId>,
    pub discord_id: Option<String>,
}
impl User {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            discord_id: None,
        }
    }
}
