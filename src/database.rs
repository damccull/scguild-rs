use std::{error::Error, fs::File, io::BufReader, path::Path};

use crate::entities::{Manufacturer, Ship};

pub fn find_ship_by_name(ships: Vec<Ship>, query: &str) -> Vec<Ship> {
    ships
        .into_iter()
        .filter(|s| s.name.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
pub 
fn find_manufacturer_by_name(manufacturers: Vec<Manufacturer>, query: &str) -> Vec<Manufacturer> {
    let query: Vec<Manufacturer> = manufacturers
        .into_iter()
        .filter(|m| m.name.to_lowercase().contains(&query.to_lowercase()))
        .collect();
    query
}
pub 
fn find_manufacturer_by_code(manufacturers: Vec<Manufacturer>, query: &str) -> Vec<Manufacturer> {
    let query: Vec<Manufacturer> = manufacturers
        .into_iter()
        .filter(|m| m.code.to_lowercase().contains(&query.to_lowercase()))
        .collect();
    query
}
pub 
fn read_ships_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Ship>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let ships = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(ships)
}
