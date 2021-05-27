use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Turret {
    size: usize,
    #[serde(default)]
    turret: bool,
    #[serde(default)]
    gimballed: bool,
    weapon_sizes: Vec<usize>,
}