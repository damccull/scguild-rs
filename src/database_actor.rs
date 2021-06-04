use std::{env, error::Error, fs::File, io::BufReader, path::Path};

use diesel::prelude::*;
use tokio::sync::{mpsc, oneshot};

use crate::{db::models as dbmodels, entities};

use crate::db::schema::manufacturers::dsl::manufacturers as manufacturer_dsl;

pub enum DatabaseMessage {
    GetManufacturers,
}

pub struct DatabaseActor {
    pub receiver: mpsc::Receiver<DatabaseMessage>,
}

impl DatabaseActor {
    pub fn new(receiver: mpsc::Receiver<DatabaseMessage>) -> Self {
        DatabaseActor { receiver }
    }

    pub fn handle_message(&mut self, msg: DatabaseMessage) {
        match msg {
            DatabaseMessage::GetManufacturers => {
                let connection = Self::establish_connection();

                let results = manufacturer_dsl
                    .load::<dbmodels::Manufacturer>(&connection)
                    .expect("Error loading manufacturers.");

                dbg!(results);
            }
        }
    }

    fn establish_connection() -> SqliteConnection {
        if (cfg!(test)) {
            let conn = SqliteConnection::establish(":memory:")
                .unwrap_or_else(|_| panic!("Error creating test database"));

            let _result = diesel_migrations::run_pending_migrations(&conn);
            conn
        } else {
            dotenv::dotenv().ok();

            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

            SqliteConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
        }
    }
}

async fn run_database_actor(mut actor: DatabaseActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

//----------------
pub fn find_ship_by_name(ships: Vec<entities::Ship>, query: &str) -> Vec<entities::Ship> {
    ships
        .into_iter()
        .filter(|s| s.name.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
// pub fn find_manufacturer_by_name(
//     manufacturers: Vec<entities::Manufacturer>,
//     query: &str,
// ) -> Vec<entities::Manufacturer> {
//     let query: Vec<entities::Manufacturer> = manufacturers
//         .into_iter()
//         .filter(|m| m.name.to_lowercase().contains(&query.to_lowercase()))
//         .collect();
//     query
// }
// pub fn find_manufacturer_by_code(
//     manufacturers: Vec<entities::Manufacturer>,
//     query: &str,
// ) -> Vec<entities::Manufacturer> {
//     let query: Vec<entities::Manufacturer> = manufacturers
//         .into_iter()
//         .filter(|m| m.code.to_lowercase().contains(&query.to_lowercase()))
//         .collect();
//     query
// }
pub fn read_ships_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<entities::Ship>, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let ships = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(ships)
}
