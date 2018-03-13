#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod models;

use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
