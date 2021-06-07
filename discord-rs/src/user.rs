use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub avatar: String,
    pub discriminator: String,
    pub public_flags: usize,
}