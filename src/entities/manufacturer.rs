use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Manufacturer {
    pub code: String,
    pub name: String,
}
