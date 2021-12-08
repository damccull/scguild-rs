use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manufacturer {
    pub id: i32,
    pub abbreviation: String,
    pub name: String,
    pub description: Option<String>,
}
