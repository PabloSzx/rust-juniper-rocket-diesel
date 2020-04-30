extern crate diesel;
extern crate diesel_demo;

use self::diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character
    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&connection, title, &body, None);
    println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";

use self::models::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn create_post<'a>(
    conn: &PgConnection,
    title: &'a str,
    body: &'a str,
    published: Option<bool>,
) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: String::from(title),
        body: String::from(body),
        published,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
