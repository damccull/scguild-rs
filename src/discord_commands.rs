use std::num::NonZeroU64;

use twilight_model::{
    application::command::{Command, CommandType},
    id::GuildId,
};
use twilight_util::builder::command::CommandBuilder;

pub fn commands() -> Vec<Command> {
    vec![CommandBuilder::new(
        "about".into(),
        "Get information about the application".into(),
        CommandType::ChatInput,
    )
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
