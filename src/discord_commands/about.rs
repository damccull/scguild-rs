use twilight_model::application::{interaction::{ApplicationCommand}, command::{Command, CommandType}};
use twilight_util::builder::command::CommandBuilder;

use super::SlashCommand;

pub struct About(pub(super) ApplicationCommand);
impl About {

}
impl SlashCommand for About{
    const NAME: &'static str = "about";

    fn define()->Command {
        CommandBuilder::new(
            "about".into(),
            "Get information about the application".into(),
            CommandType::ChatInput,
        )
        .build()
    }

    fn run< 'async_trait>(self)-> core::pin::Pin<Box<dyncore::future::Future<Output=anyhow::Result<()> > + core::marker::Send+ 'async_trait> >whereSelf: 'async_trait {
        let ctx = Interaction::new(self.0);
    }
}