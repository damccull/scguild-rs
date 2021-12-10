use twilight_model::application::{interaction::{ApplicationCommand, Interaction}, command::{Command, CommandType}, callback::{InteractionResponse, CallbackData}};
use twilight_util::builder::command::CommandBuilder;

use crate::discord::SlashCommand;

use crate::discord::api::DiscordApiError;

pub struct About(pub ApplicationCommand);
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

    // fn run< 'async_trait>(self)-> core::pin::Pin<Box<dyncore::future::Future<Output=anyhow::Result<()> > + core::marker::Send+ 'async_trait> >whereSelf: 'async_trait {
    //     let ctx = Interaction::new(self.0);
    // }
}

