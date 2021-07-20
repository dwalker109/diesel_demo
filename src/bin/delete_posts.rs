use diesel::prelude::*;
use diesel_demo::establish_connection;
use std::env::args;

fn main() {
    use diesel_demo::schema::posts::dsl::*;

    let target = args().nth(1).expect("need a delete target");
    let pattern = format!("%{}%", target);

    let conn = establish_connection();
    let qty_deleted = diesel::delete(posts.filter(title.like(&pattern)))
        .execute(&conn)
        .expect("error deleting posts");

    println!("Deleted {} posts!", qty_deleted);
}
