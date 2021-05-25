use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Turret {
    size: usize,
    turret: bool,
    gimballed: bool,
    weapon_sizes: Vec<usize>,
}