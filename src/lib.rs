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

use self::models::{NewPost, Post};
use schema::posts;

pub fn create_post(conn: &PgConnection, title: &str, body: &str) -> Post {
    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("error saving new post")
}
