use crate::snowflake::Snowflake;

/// The base "command" model.
pub struct ApplicationCommand {
    /// The unique ID of the command.
    id: Snowflake,
    /// The unique ID of the parent application.
    application_id: Snowflake,
    /// 1-32 lowercase character name matching `^[\w-]{1,32}$`
    name: String,
    /// 1-100 character description
    description: String,
    /// Vec of parameters for the command.
    options: Option<Vec<ApplicationCommandOption>>,
    /// Whether the command is enabled by defualt when the app is added to the guild.
    default_permission: Option<bool>,
}

///
pub struct ApplicationCommandOption {
    /// An `ApplicationCommandOptionType` representing the kind of command option this is.
    r#type: ApplicationCommandOptionType,
    /// 1-32 lowercase character name matching ^[\w-]{1,32}$
    name: String,
    /// 1-100 character description.
    description: String,
    /// Whether or not the parameter is required. Default is false on Discord.
    required: Option<bool>,
    /// Choices for the user to pick from. String and int types.
    choices: Option<Vec<ApplicationCommandOptionChoice>>,
    /// Nested options for when the option is a subcommand or subcommand group type.
    options: Option<Box<ApplicationCommandOption>>,
}

/// If choices are specified for a command, they are the only valid choices a user can pick.
pub struct ApplicationCommandOptionChoice {
    /// 1-100 character choice name
    name: String,
    /// Value of the choice, up to 100 characters if it is a string
    value: String,
}

#[repr(u8)] // Ensure this enum is stored and represented as a u8
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
}
