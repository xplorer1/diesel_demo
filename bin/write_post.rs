extern crate diesel;
extern crate diesel_demo;

use std::io::{stdin, Read};
use self::diesel_demo::*;

fn main() {
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();

    println!("\nOk! Let's write {}(Press {} when finished!)\n", title, EOF);
    let mut body = String::new();
    stdin().read_line(&mut body).unwrap();

    let post = create_post(&connection, title: &'a str, body: &'a str);
    println!("\nSaved draft {} with id {}", title post.id);
}

#[cfg(not(windows))]
const EOF:&'static str = "CTRL+D";

#[cfg(windows)]
const EOF:&'static str = "CTRL+Z";

pub fn create_post<'a>(title: &'a str, body: &'a str) -> Post {
    use schema::posts;
    let new_post = NewPost {
        title: title,
        body: body,
    };
}