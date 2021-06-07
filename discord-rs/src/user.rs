use serde::{Deserialize, Serialize};

use crate::snowflake::Snowflake;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// The user's ID
    pub id: Snowflake,
    /// The user's username, not unique across the platform
    pub username: String,
    /// The user's 4-digit discord-tag
    pub discriminator: String,
    /// The user's avatar hash
    pub avatar: Option<String>,
    /// Whether the user belongs to an Oauth2 application
    pub bot: Option<bool>,
    /// Whether the user is an Official Discord System user (urgent message system)
    pub system: Option<bool>,
    /// Whether the user has two factor authentication enabled on their account
    pub mfa_enabled: Option<bool>,
    /// The user's chosen language option
    pub locale: Option<String>,
    /// Whether the user's email address has been verified
    pub verified: Option<bool>,
    /// The user's email address
    pub email: Option<String>,
    /// The flags on the user's account
    pub flags: Option<UserFlags>,
    /// The type of Nitro subscription on the user's account
    pub premium_type: Option<NitroType>,
    /// The public flags on the user's account
    pub public_flags: Option<UserFlags>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum NitroType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct UserFlags: u32 {
        const NONE = 0b00000000; // 0
        const DISCORD_EMPLOYEE = 0b00000001; // 1 << 0
        const PARTNERED_SERVER_OWNER = 0b00000010; // 1 << 1
        const HYPE_SQUAD_EVENTS = 0b00000100; // 1 << 2
        const BUG_HUNTER_LEVEL1 = 0b00001000; // 1 << 3
        const HOUSE_BRAVERY = 0b01000000; // 1 << 6
        const HOUSE_BRILLIANCE = 0b10000000; // 1 << 7
        const HOUSE_BALANCE = 0b100000000; // 1 << 8
        const EARLY_SUPPORTER = 0b1000000000; // 1 << 9
        const TEAM_USER = 0b10000000000; // 1 << 10
        const BUG_HUNTER_LEVEL2 = 0b100000000000000; // 1 << 14
        const VERIFIED_BOT = 0b10000000000000000; // 1 << 16
        const EARLY_VERIFIED_BOT_DEVELOPER = 0b100000000000000000; // 1 << 17
        const DISCORD_CERTIFIED_MODERATOR = 0b1000000000000000000; // 1 << 18
    }
}
