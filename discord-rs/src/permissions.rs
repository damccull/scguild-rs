use crate::snowflake::{self, Snowflake};

/// Returned when fetching the permissions for a command in a guild.
pub struct GuildApplicationCommandPermissions {
    /// The ID of the command
    pub id: Snowflake,
    /// The ID of the application the command belongs to
    pub application_id: Snowflake,
    /// The ID of the guild
    pub guild_id: Snowflake,
    /// The permissions for the command in the guild
    pub permissions: Vec<ApplicationCommandPermissions>,
}

/// Application command permissions allow you to enable or disable
/// commands for specific users or roles within a guild.
pub struct ApplicationCommandPermissions {
    /// The ID of the role or user
    pub id: Snowflake,
    /// If the permission is for a Role or User type
    pub r#type: ApplicationCommandPermissionType,
    /// True to allow the command, false to disallow
    pub permission: bool,
}

#[repr(u8)]
pub enum ApplicationCommandPermissionType {
    Role = 1,
    User = 2
}
