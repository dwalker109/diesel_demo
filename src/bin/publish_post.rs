use diesel::prelude::*;
use diesel_demo::{establish_connection, models::Post};
use std::env::args;

fn main() {
    use diesel_demo::schema::posts::dsl;

    let id = args()
        .nth(1)
        .expect("requires a post id to publish")
        .parse::<i32>()
        .expect("invalid id");

    let conn = establish_connection();

    let post = diesel::update(dsl::posts.find(&id))
        .set(dsl::published.eq(true))
        .get_result::<Post>(&conn)
        .expect("unable to find post");

    println!("Published post {}!", post.id);
}
