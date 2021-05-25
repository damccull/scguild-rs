use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize="camelCase"))]
pub struct Manufacturer {
    code: String,
    name: String,
}