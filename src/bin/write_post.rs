use diesel::prelude::*;
use std::io::{stdin, Read};

use diesel_demo::{
    establish_connection,
    models::{NewPost, Post},
    schema::posts,
};

fn main() {
    let conn = establish_connection();

    println!("Title?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop newline

    println!("Body? (CTRL+D when finished)");
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let new_post = NewPost { title, body: &body };

    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(&conn)
        .expect("error saving new post");

    println!("Saved draft with id: {}", post.id);
}
