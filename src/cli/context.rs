// Objects representing a user created local database
use chrono::prelude::*;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Context {
    pub version: String,
    pub environment: List<Environment>,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize)]
pub struct Environment {
    target: String,
    org_id: String,
}