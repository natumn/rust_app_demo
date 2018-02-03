pub mod schema;
pub mod models;

#[macro_use]
extern crate rust_app_demo;
extern crate diesel;
extern crate dotenv;

use self::rust_app_demo::*;
use self::models::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use std::io::{stdin, Read};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &pgConnection, title: &'a str, body: &'a str) -> Post {
    use schma::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel:insert_into(post::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

fn main() {
    use rust_app_demo::schema::posts::dsl::*;

    let conn = establish_connection();

    println!("What would you like your title to be?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)];
    println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    let mut body = String::new();
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(&conn, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);

    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&conn)
        .expect("Error loading posts");
    
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-------------\n");
        println!("{}", post.body);
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
