use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::database::schema::manufacturers;
use crate::database::schema::manufacturers::dsl::manufacturers as manufacturer_dsl;

#[derive(Clone, Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "manufacturers"]
pub struct Manufacturer {
    pub id: i32,
    pub abbreviation: String,
    pub name: String,
    pub description: Option<String>,
}