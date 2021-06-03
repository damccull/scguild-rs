#![allow(unused)]

use std::{collections::HashMap, error::Error, fs::File, io::BufReader, net::TcpListener, path::Path};

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use entities::Ship;

use crate::entities::Manufacturer;

mod crypto;
mod entities;
mod database;

use norseline::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    test_stuff();


    //------- TEMPORARILY DISABLED: This enables the web server
    // Create a TcpListener to pass to the server. This allows for easy integration testing.
    // let listener = TcpListener::bind("127.0.0.1:4201").expect("Failed to bind random port");
    // run(listener)?.await
    Ok(())
}

fn test_stuff() {
    match database::read_ships_from_file("ships.json") {
        Ok(ships) => {
            let ship_class_names: Vec<String> =
                ships.iter().map(|s| s.class_name.clone()).collect();
            let manufacturers: Vec<Manufacturer> =
                ships.iter().fold(Vec::<Manufacturer>::new(), |mut acc, s| {
                    if acc.iter().find(|m| m.name == s.manufacturer.name).is_none() {
                        acc.push(s.manufacturer.clone());
                    }
                    acc
                });
            let ships_by_manufacturer: HashMap<String, Vec<String>> =
                ships
                    .iter()
                    .fold(HashMap::<String, Vec<String>>::new(), |mut acc, ship| {
                        acc.entry(ship.manufacturer.name.clone())
                            .or_insert_with(Vec::<String>::new)
                            .push(ship.name.clone());
                        acc
                    });
            //let query = find_manufacturer_by_code(manufacturers, &String::from("cn"));
            let query = database::find_ship_by_name(ships, "stalker");
            dbg!(query);
            // dbg!(ships_by_manufacturer.get_key_value("Anvil"));
        }
        Err(e) => panic!("Error reading ships.json: {:?}", e),
    }
}

