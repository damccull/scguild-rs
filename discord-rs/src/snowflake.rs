use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[derive(Eq, PartialEq, Hash)]
pub struct Snowflake(u64);
