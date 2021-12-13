use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello to other members")]
pub struct HelloCommand {
    /// Message to send
    pub message: String,
    /// User to send the message to
    pub user: Option<ResolvedUser>,
}
