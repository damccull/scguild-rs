use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manufacturer {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
