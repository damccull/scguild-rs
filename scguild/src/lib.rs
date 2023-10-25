use poise::Command;

pub mod fleet;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Context<'a> = poise::Context<'a, Data, anyhow::Error>;
pub type CommandList = Vec<Command<Data, anyhow::Error>>;
