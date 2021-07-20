pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;

use diesel::{pg::PgConnection, prelude::*};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect("error connecting to database")
}
