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
    pilot_hard_points: Vec<Hardpoint>,
    manned_turrets: Vec<Turret>,
    remote_turrets: Vec<Turret>,
    insurance: Insurance,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DamageBeforeDestruction {
    nose: f32,
    body: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct DamageBeforeDetach {
    // All these are individually named because of bad naming in json source.
    #[serde(rename(deserialize = "Canopy"))]
    canopy: f32,
    #[serde(rename(deserialize = "Wing_Right"))]
    wing_right: f32,
    #[serde(rename(deserialize = "Wing_Flap_Right"))]
    wing_flap_right: f32,
    #[serde(rename(deserialize = "WingTip_Right"))]
    wingtip_right: f32,
    #[serde(rename(deserialize = "Wing_Left"))]
    wing_left: f32,
    #[serde(rename(deserialize = "Wing_Flap_Left"))]
    wing_flap_left: f32,
    #[serde(rename(deserialize = "WingTip_Left"))]
    wingtip_left: f32,
    #[serde(rename(deserialize = "HullTail_Right"))]
    hulltail_right: f32,
    #[serde(rename(deserialize = "Right_Tail_Fin_Flap"))]
    right_tail_fin_flap: f32,
    #[serde(rename(deserialize = "HullTail_Left"))]
    hulltail_left: f32,
    #[serde(rename(deserialize = "Left_Tail_Fin_Flap"))]
    left_tail_fin_flap: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct FlightCharacteristics {
    scm_speed: f32,
    max_speed: f32,
    zero_to_scm: f32,
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
    intake_to_main_fuel_ration: f32,
    intake_to_tank_capacity_ration: f32,
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
    fixed: bool,
    weapon_sizes: Vec<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase", serialize = "camelCase"))]
pub struct Insurance {
    standard_claim_time: f32,
    expedited_claim_time: f32,
    expedited_cost: f32,
}
