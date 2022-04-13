use twilight_model::application::command::CommandOption;
use twilight_util::builder::command::{StringBuilder, SubCommandBuilder};

use crate::discord::DiscordSubcommand;

#[derive(Clone, Debug)]
pub struct ListCommand {
    user_name: Option<String>,
}
impl DiscordSubcommand for ListCommand {
    const NAME: &'static str = "list";
    const DESCRIPTION: &'static str = "Displays all the ships in your fleet to just yourself.";

    fn register() -> CommandOption {
        SubCommandBuilder::new(Self::NAME.into(), Self::DESCRIPTION.into())
            .option(
                StringBuilder::new("ship_model".into(), "type the ship model".into())
                    .required(true)
                    .autocomplete(true),
            )
            .option(StringBuilder::new(
                "ship_name".into(),
                "type the name of the ship".into(),
            ))
            .build()
    }
}
impl ListCommand {}
