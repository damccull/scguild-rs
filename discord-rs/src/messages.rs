use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum MessageComponentType {
    ActionRow = 1,
    Button = 2,
}
