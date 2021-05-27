use core::f64;

use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;

use super::{Manufacturer, Turret};

//TODO Consider a custom visitor for f64 after all considering the number of needed options.

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
#[serde(default)]
pub struct Ship {
    class_name: String,
    name: String,
    description: String,
    career: String,
    role: String,
    size: usize,
    cargo: usize,
    crew: usize,
    weapon_crew: usize,
    operations_crew: usize,
    mass: f64,
    is_spaceship: bool,
    manufacturer: Manufacturer,
    damage_before_destruction: DamageBeforeDestruction,
    damage_before_detach: DamageBeforeDetach,
    flight_characteristics: FlightCharacteristics,
    propulsion: Propulsion,
    quantum_travel: QuantumTravel,
    pilot_hardpoints: Vec<Hardpoint>,
    manned_turrets: Vec<Turret>,
    remote_turrets: Vec<Turret>,
    insurance: Option<Insurance>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DamageBeforeDestruction {
    // Aliases because json does not normalize key casing.
    #[serde(alias = "Nose")]
    #[serde(alias = "nose")]
    nose: Option<f64>,
    #[serde(alias = "Body")]
    #[serde(alias = "body")]
    body: Option<f64>,
    #[serde(alias = "Front")]
    #[serde(alias = "front")]
    front: Option<f64>,
    #[serde(alias = "Rear")]
    #[serde(alias = "rear")]
    rear: Option<f64>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct DamageBeforeDetach {
    // All these are individually named because of bad naming in json source.
    #[serde(alias = "Canopy")]
    #[serde(alias = "canopy")]
    canopy: Option<f64>,
    #[serde(alias = "Wing_Right")]
    #[serde(alias = "wing_right")]
    wing_right: Option<f64>,
    #[serde(alias = "Wing_Flap_Right")]
    #[serde(alias = "wing_flap_right")]
    wing_flap_right: Option<f64>,
    #[serde(alias = "WingTip_Right")]
    #[serde(alias = "wingtip_right")]
    wingtip_right: Option<f64>,
    #[serde(alias = "Wing_Left")]
    #[serde(alias = "wing_left")]
    wing_left: Option<f64>,
    #[serde(alias = "Wing_Flap_Left")]
    #[serde(alias = "wing_flap_left")]
    wing_flap_left: Option<f64>,
    #[serde(alias = "WingTip_Left")]
    #[serde(alias = "wingtip_left")]
    wingtip_left: Option<f64>,
    #[serde(alias = "HullTail_Right")]
    #[serde(alias = "hulltail_right")]
    hulltail_right: Option<f64>,
    #[serde(alias = "Right_Tail_Fin_Flap")]
    #[serde(alias = "right_tail_fin_flap")]
    right_tail_fin_flap: Option<f64>,
    #[serde(alias = "HullTail_Left")]
    #[serde(alias = "hulltail_left")]
    hulltail_left: Option<f64>,
    #[serde(alias = "Left_Tail_Fin_Flap")]
    #[serde(alias = "left_tail_fin_flap")]
    left_tail_fin_flap: Option<f64>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct FlightCharacteristics {
    scm_speed: f64,
    max_speed: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    zero_to_scm: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    zero_to_max: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    scm_to_zero: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    max_to_zero: f64,
    acceleration: ThrustDirectionValue,
    acceleration_g: ThrustDirectionValue,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct ThrustDirectionValue {
    #[serde(deserialize_with = "f64_deserializer")]
    main: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    retro: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    vtol: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    maneuvering: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Propulsion {
    fuel_capacity: f64,
    fuel_intake_rate: f64,
    fuel_usage: ThrustDirectionValue,
    thrust_capacity: ThrustDirectionValue,
    #[serde(deserialize_with = "f64_deserializer")]
    intake_to_main_fuel_ratio: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    intake_to_tank_capacity_ratio: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    time_for_intakes_to_fill_tank: f64,
    #[serde(deserialize_with = "f64_deserializer")]
    maneuvering_time_till_empty: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct QuantumTravel {
    speed: f64,
    spool_time: f64,
    fuel_capacity: f64,
    range: f64,
    port_olisar_to_arc_corp_time: f64,
    port_olisar_to_arc_corp_fuel: f64,
    port_olisar_to_arc_corp_and_back: f64,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Hardpoint {
    size: usize,
    #[serde(default)]
    fixed: bool,
    #[serde(default)]
    gimballed: bool,
    weapon_sizes: Vec<usize>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Insurance {
    standard_claim_time: f64,
    expedited_claim_time: f64,
    expedited_cost: f64,
}

fn f64_deserializer<'de, D>(d: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Value = Deserialize::deserialize(d)?;
    match s {
        Value::String(s) if s == "Infinity" => Ok(f64::INFINITY),
        Value::String(s) if s == "NaN" => Ok(f64::NAN),
        Value::Number(s) => s.as_f64().ok_or_else(|| Error::custom("failed to parse")),
        _ => Err(Error::custom(format!(
            "invalid type: expected float. found: {}",
            s
        ))),
    }
}
