use crate::snowflake::{self, Snowflake};

/// Returned when fetching the permissions for a command in a guild.
pub struct GuildApplicationCommandPermissions {
    /// The ID of the command
    id: Snowflake,
    /// The ID of the application the command belongs to
    application_id: Snowflake,
    /// The ID of the guild
    guild_id: Snowflake,
    /// The permissions for the command in the guild
    permissions: Vec<ApplicationCommandPermissions>,
}

/// Application command permissions allow you to enable or disable
/// commands for specific users or roles within a guild.
pub struct ApplicationCommandPermissions {
    /// The ID of the role or user
    id: Snowflake,
    /// If the permission is for a Role or User type
    r#type: ApplicationCommandPermissionType,
    /// True to allow the command, false to disallow
    permission: bool,
}

#[repr(u8)]
pub enum ApplicationCommandPermissionType {
    Role = 1,
    User = 2
}
