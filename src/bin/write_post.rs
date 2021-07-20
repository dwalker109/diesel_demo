use std::io::{stdin, Read};

use diesel_demo::{create_post, establish_connection};

fn main() {
    let conn = establish_connection();

    println!("Title?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop newline

    println!("Body? (CTRL+D when finished)");
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&conn, title, &body);
    println!("Saved draft with id: {}", post.id);
}
