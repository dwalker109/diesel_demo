// extern crate diesel;
// extern crate diesel_demo;

use diesel::prelude::*;
use diesel_demo::models::*;
use diesel_demo::*;

fn main() {
    use diesel_demo::schema::posts::dsl::*;

    let conn = establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&conn)
        .expect("error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("{}", post.body);
    }
}
