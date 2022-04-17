use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manufacturer {
    /// The manufacturer's id.
    pub id: Uuid,
    /// The optional short-code or abbreviation for the manufacturer.
    pub code: Option<String>,
    /// The full official name of the manufacturer.
    pub name: String,
    /// An optional description of the company.
    pub description: Option<String>,
}
