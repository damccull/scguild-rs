use async_trait::async_trait;
use twilight_model::application::{
    callback::InteractionResponse,
    command::Command,
    interaction::{ApplicationCommand, Interaction},
};

use self::{api::DiscordApiError, commands::About};

pub mod api;
mod commands;
// mod interaction;

#[async_trait]
trait SlashCommand {
    /// Name of the command.
    /// Required to match incoming interactions.
    const NAME: &'static str;

    /// Command definition
    fn define() -> Command;

    async fn api_handler(interaction: Interaction) -> Result<InteractionResponse, DiscordApiError>;

    // /// Run the command, self should be an [`ApplicationCommand`].
    // async fn run(self) -> Result<()>;
}

pub enum DiscordCommand {
    About(About),
    // Fleet(Fleet),
    // Ships(Ships),
    // AddShip(AddShip),
    // RemoveShip(RemoveShip),
}
impl DiscordCommand {
    pub fn get(cmd: ApplicationCommand) -> Option<Self> {
        match cmd.data.name.as_str() {
            About::NAME => Some(Self::About(About(cmd))),
            // Fleet::NAME => Some(Self::Fleet(Fleet(cmd))),
            // Ships::NAME => Some(Self::Ships(Ships(cmd))),
            // AddShip::NAME => Some(Self::AddShip(AddShip(cmd))),
            // RemoveShip::NAME => Some(Self::RemoveShip(RemoveShip(cmd))),
            _ => None,
        }
    }
}

pub fn commands() -> [Command; 1] {
    [
        About::define(),
        // Fleet::define(),
        // Ships::define(),
        // AddShip::define,
        // RemoveShip::define(),
    ]

    // [Command {
    //     id: None,
    //     guild_id: None,
    //     application_id: None,
    //     name: "about".to_string(),
    //     description: "Gets information about the application.".to_string(),
    //     options: vec![],
    //     kind: CommandType::ChatInput,
    //     default_permission: None,
    // }]
}
