use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ShipModel {
    /// The id of the ship model.
    pub id: Uuid,
    /// The game data internal class name.
    pub class_name: String,
    /// The friendly string name.
    pub name: String,
    /// The id of the manufacturer.
    pub manufacturer_id: Uuid,
    /// A description of the ship model.
    pub description: Option<String>,
    /// The focused purpose of the ship model.
    pub focus: Option<String>, // Parse Description, Probably with regex
    /// The career of the ship model.
    pub career: String,
    /// The role of the ship model.
    pub role: String,
    /// The size of the ship.
    pub size: i32,
    /// The maximum cargo the ship can carry.
    pub cargo: i32,
    /// The suggested number of crew for the ship.
    pub crew: i32,
    /// The number of weapons crew to fully man all the ship's weapons.
    pub weapon_crew: i32,
    /// Unknown - likely the number of crew required to fully man non-weapons systems,
    /// not including command and pilot crew.
    pub operations_crew: i32,
    /// The total mass of the ship model.
    pub mass: i32,
    /// Whether or not the entity is a spaceship.
    pub is_spaceship: bool,
    /// Whether or not the entity is a ground vehicle.
    pub is_vehicle: bool,
    /// Whether or not the entity is a gravlev vehicle.
    pub is_gravlev: bool,
}
