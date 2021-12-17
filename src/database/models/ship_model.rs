use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShipModel {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}
