use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize="camelCase"))]
pub struct Manufacturer {
    code: String,
    name: String,
}