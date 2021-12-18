use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

use crate::discord::commands::{FleetCommand, HelloCommand};

pub mod api;
mod commands;

pub fn commands() -> Vec<Command> {
    vec![
        // About::define(),
        FleetCommand::create_command().into(),
        // Wishlist::define(),
        HelloCommand::create_command().into(),
    ]
}
