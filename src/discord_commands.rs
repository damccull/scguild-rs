use std::num::{NonZeroU16, NonZeroU64};

use twilight_model::{
    application::command::{ChoiceCommandOptionData, Command, CommandOption, CommandType},
    id::GuildId,
};
use twilight_util::builder::command::{BooleanBuilder, CommandBuilder, StringBuilder};

pub fn commands() -> Vec<Command> {
    vec![CommandBuilder::new(
        "about".into(),
        "Get information about the application".into(),
        CommandType::ChatInput,
    )
    .guild_id(GuildId {
        0: NonZeroU64::new(751806938190446710).unwrap(),
    })
    .build()]

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
