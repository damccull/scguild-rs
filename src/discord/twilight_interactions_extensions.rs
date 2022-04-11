use std::convert::TryFrom;
use anyhow::{Result, Ok};

use twilight_model::application::interaction::{Interaction, application_command::CommandDataOption};

#[derive(Clone, Debug)]
pub enum InteractionAutocompleteOption<T> {
    /// The interaction is sent completely, as if a complete command.
    Complete(T),
    /// The interaction is sent partially, as if being autocompleted.
    Partial(String),
}
