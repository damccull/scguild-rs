use core::f32;

use serde::Deserialize;

use super::{Manufacturer, Turret};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
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
    mass: f32,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DamageBeforeDestruction {
    // Aliases because json does not normalize key casing.
    #[serde(alias = "Nose")]
    #[serde(alias = "nose")]
    nose: Option<f32>,
    #[serde(alias = "Body")]
    #[serde(alias = "body")]
    body: Option<f32>,
    #[serde(alias = "Front")]
    #[serde(alias = "front")]
    front: Option<f32>,
    #[serde(alias = "Rear")]
    #[serde(alias = "rear")]
    rear: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct DamageBeforeDetach {
    // All these are individually named because of bad naming in json source.
    #[serde(alias = "Canopy")]
    #[serde(alias = "canopy")]
    canopy: Option<f32>,
    #[serde(alias = "Wing_Right")]
    #[serde(alias = "wing_right")]
    wing_right: Option<f32>,
    #[serde(alias = "Wing_Flap_Right")]
    #[serde(alias = "wing_flap_right")]
    wing_flap_right: Option<f32>,
    #[serde(alias = "WingTip_Right")]
    #[serde(alias = "wingtip_right")]
    wingtip_right: Option<f32>,
    #[serde(alias = "Wing_Left")]
    #[serde(alias = "wing_left")]
    wing_left: Option<f32>,
    #[serde(alias = "Wing_Flap_Left")]
    #[serde(alias = "wing_flap_left")]
    wing_flap_left: Option<f32>,
    #[serde(alias = "WingTip_Left")]
    #[serde(alias = "wingtip_left")]
    wingtip_left: Option<f32>,
    #[serde(alias = "HullTail_Right")]
    #[serde(alias = "hulltail_right")]
    hulltail_right: Option<f32>,
    #[serde(alias = "Right_Tail_Fin_Flap")]
    #[serde(alias = "right_tail_fin_flap")]
    right_tail_fin_flap: Option<f32>,
    #[serde(alias = "HullTail_Left")]
    #[serde(alias = "hulltail_left")]
    hulltail_left: Option<f32>,
    #[serde(alias = "Left_Tail_Fin_Flap")]
    #[serde(alias = "left_tail_fin_flap")]
    left_tail_fin_flap: Option<f32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct FlightCharacteristics {
    scm_speed: f32,
    max_speed: f32,
    #[serde(default)]
    zero_to_scm: f32,
    #[serde(default)]
    zero_to_max: f32,
    scm_to_zero: f32,
    max_to_zero: f32,
    acceleration: ThrustDirectionValue,
    acceleration_g: ThrustDirectionValue,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct ThrustDirectionValue {
    main: f32,
    retro: f32,
    vtol: f32,
    maneuvering: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Propulsion {
    fuel_capacity: f32,
    fuel_intake_rate: f32,
    fuel_usage: ThrustDirectionValue,
    thrust_capacity: ThrustDirectionValue,
    intake_to_main_fuel_ratio: f32,
    intake_to_tank_capacity_ratio: f32,
    time_for_intakes_to_fill_tank: f32,
    maneuvering_time_till_empty: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct QuantumTravel {
    speed: f32,
    spool_time: f32,
    fuel_capacity: f32,
    range: f32,
    port_olisar_to_arc_corp_time: f32,
    port_olisar_to_arc_corp_fuel: f32,
    port_olisar_to_arc_corp_and_back: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Hardpoint {
    size: usize,
    #[serde(default)]
    fixed: bool,
    #[serde(default)]
    gimballed: bool,
    weapon_sizes: Vec<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Insurance {
    standard_claim_time: f32,
    expedited_claim_time: f32,
    expedited_cost: f32,
}
