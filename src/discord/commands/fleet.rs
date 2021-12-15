use async_trait::async_trait;
use twilight_interactions::command::{CommandInputData, CommandModel, CreateCommand, ResolvedUser};
use twilight_model::application::{
    callback::{CallbackData, InteractionResponse},
    interaction::ApplicationCommand,
};

use crate::discord::{api::DiscordApiError, SlashCommand};

#[allow(clippy::large_enum_variant)]
#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "fleet", desc = "Manage or view your fleet, or show it off.")]
pub enum FleetCommand {
    #[command(name = "add")]
    Add(AddCommand),
    #[command(name = "list")]
    List(ListCommand),
    #[command(name = "remove")]
    Remove(RemoveCommand),
    #[command(name = "rename")]
    Rename(RenameCommand),
    #[command(name = "show")]
    Show(ShowCommand),
}
impl FleetCommand {
    pub const NAME: &'static str = "fleet";
}

#[async_trait]
impl SlashCommand for FleetCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET", skip(cmd))]
    async fn handler(cmd: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError> {
        let x: CommandInputData = cmd.data.clone().into();
        match FleetCommand::from_interaction(x) {
            Ok(subcommand) => match subcommand {
                FleetCommand::Add(add_command) => add_command.handle(cmd).await,
                FleetCommand::List(_) => todo!(),
                FleetCommand::Remove(_) => todo!(),
                FleetCommand::Rename(_) => todo!(),
                FleetCommand::Show(show_command) => show_command.handle(cmd).await,
            },
            Err(e) => {
                return Err(DiscordApiError::UnsupportedCommand(format!(
                    "Something went wrong parsing the interaction: {}",
                    e
                )));
            }
        }
    }

    #[tracing::instrument(name = "Discord Interaction - FLEET ADD AUTOCOMPLETE", skip(cmd))]
    async fn autocomplete_handler(
        cmd: &ApplicationCommand,
    ) -> Result<InteractionResponse, DiscordApiError> {
        let x: CommandInputData = cmd.data.clone().into();
        dbg!(&x);
        let partial = AddCommandPartial::from_interaction(x);
        dbg!(&partial);
        // match partial {
        //     Ok(subcommand) => match subcommand {
        //         FleetCommand::Add(add_command) => add_command.handle(cmd).await,
        //         _ => return Err(DiscordApiError::AutocompleteUnsupported),
        //     },
        //     Err(e) => {
        //         return Err(DiscordApiError::UnsupportedCommand(format!(
        //             "Something went wrong parsing the interaction: {}",
        //             e
        //         )));
        //     }
        // }
        return Err(DiscordApiError::UnsupportedCommand(format!(
            "Something went wrong parsing the interaction:",
        )));
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "add", desc = "Add a ship to your fleet.")]
pub struct AddCommand {
    /// The model of ship you want to add.
    #[command(autocomplete = true)]
    pub ship_model: String,
    /// The name of the ship. (Optional)
    pub ship_name: Option<String>,
}

impl AddCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET ADD", skip(self))]
    async fn handle(
        &self,
        cmd: &ApplicationCommand,
    ) -> Result<InteractionResponse, DiscordApiError> {
        let ship_model = self.ship_model.to_owned();
        let ship_name = match self.ship_name.to_owned() {
            Some(name) => format!(" named _{}_", name),
            None => "".into(),
        };

        unsafe {
            fakedb.push(Ship {
                model: ship_model.to_owned(),
                name: self.ship_name.clone(),
            });
        }

        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackData {
                allowed_mentions: None,
                flags: None,
                tts: None,
                content: Some(format!(
                    "Adding a {}{} to the fleet.",
                    ship_model, ship_name
                )),
                embeds: Default::default(),
                components: Default::default(),
            },
        ))
    }
}

#[derive(CommandModel, Debug)]
#[command(partial = true)]
pub struct AddCommandPartial {
    /// The model of ship you want to add.
    pub ship_model: String,
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(
    name = "list",
    desc = "Privately list the ships in your, or the specified user's, fleet."
)]
pub struct ListCommand {
    /// The user who's fleet you'd like to see. (Optional)
    pub user: Option<ResolvedUser>,
}

impl ListCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET")]
    async fn handler(_cmd: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError> {
        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackData {
                allowed_mentions: None,
                flags: None,
                tts: None,
                content: Some("Privately perusing the fleet.".into()),
                embeds: Default::default(),
                components: Default::default(),
            },
        ))
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "remove", desc = "Remove a ship from your fleet.")]
pub struct RemoveCommand {}

impl RemoveCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET")]
    async fn handler(_cmd: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError> {
        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackData {
                allowed_mentions: None,
                flags: None,
                tts: None,
                content: Some("Removing a ship from the fleet.".into()),
                embeds: Default::default(),
                components: Default::default(),
            },
        ))
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "rename", desc = "Remame a ship in your fleet.")]
pub struct RenameCommand {}

impl RenameCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET")]
    async fn handler(_cmd: &ApplicationCommand) -> Result<InteractionResponse, DiscordApiError> {
        Ok(InteractionResponse::ChannelMessageWithSource(
            CallbackData {
                allowed_mentions: None,
                flags: None,
                tts: None,
                content: Some("Renaming a ship in the fleet.".into()),
                embeds: Default::default(),
                components: Default::default(),
            },
        ))
    }
}

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "show", desc = "Show your fleet to the channel.")]
pub struct ShowCommand {
    /// This is a dummy option. Set it true or false. It's just here temporarily due to a bug.
    dummy: bool,
}

impl ShowCommand {
    #[tracing::instrument(name = "Discord Interaction - FLEET SHOW")]
    async fn handle(
        &self,
        _cmd: &ApplicationCommand,
    ) -> Result<InteractionResponse, DiscordApiError> {
        unsafe {
            Ok(InteractionResponse::ChannelMessageWithSource(
                CallbackData {
                    allowed_mentions: None,
                    flags: None,
                    tts: None,
                    content: Some(format!("Showing off the fleet.\n```\n{:?}\n```", fakedb)),
                    embeds: Default::default(),
                    components: Default::default(),
                },
            ))
        }
    }
}

#[derive(Debug)]
struct Ship {
    model: String,
    name: Option<String>,
}

static mut fakedb: Vec<Ship> = Vec::new();
