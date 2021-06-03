#![allow(unused)]

use std::{collections::HashMap, error::Error, fs::File, io::BufReader, net::TcpListener, path::Path};

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use entities::Ship;

use crate::entities::Manufacturer;

mod crypto;
mod entities;

use norseline::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a TcpListener to pass to the server. This allows for easy integration testing.
    let listener = TcpListener::bind("127.0.0.1:4201").expect("Failed to bind random port");
    run(listener)?.await

    // match read_ships_from_file("ships.json") {
    //     Ok(ships) => {
    //         let ship_class_names: Vec<String> =
    //             ships.iter().map(|s| s.class_name.clone()).collect();
    //         let manufacturers: Vec<Manufacturer> =
    //             ships.iter().fold(Vec::<Manufacturer>::new(), |mut acc, s| {
    //                 if acc.iter().find(|m| m.name == s.manufacturer.name).is_none() {
    //                     acc.push(s.manufacturer.clone());
    //                 }
    //                 acc
    //             });
    //         let ships_by_manufacturer: HashMap<String, Vec<String>> =
    //             ships
    //                 .iter()
    //                 .fold(HashMap::<String, Vec<String>>::new(), |mut acc, ship| {
    //                     acc.entry(ship.manufacturer.name.clone())
    //                         .or_insert_with(Vec::<String>::new)
    //                         .push(ship.name.clone());
    //                     acc
    //                 });
    //         //let query = find_manufacturer_by_code(manufacturers, &String::from("cn"));
    //         let query = find_ship_by_name(ships, "stalker");
    //         dbg!(query);
    //         // dbg!(ships_by_manufacturer.get_key_value("Anvil"));
    //     }
    //     Err(e) => panic!("Error reading ships.json: {:?}", e),
    // }
}

fn find_ship_by_name(ships: Vec<Ship>, query: &str) -> Vec<Ship> {
    ships
        .into_iter()
        .filter(|s| s.name.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

fn find_manufacturer_by_name(manufacturers: Vec<Manufacturer>, query: &str) -> Vec<Manufacturer> {
    let query: Vec<Manufacturer> = manufacturers
        .into_iter()
        .filter(|m| m.name.to_lowercase().contains(&query.to_lowercase()))
        .collect();
    query
}

fn find_manufacturer_by_code(manufacturers: Vec<Manufacturer>, query: &str) -> Vec<Manufacturer> {
    let query: Vec<Manufacturer> = manufacturers
        .into_iter()
        .filter(|m| m.code.to_lowercase().contains(&query.to_lowercase()))
        .collect();
    query
}

fn read_ships_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Ship>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let ships = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(ships)
}
