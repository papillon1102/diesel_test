#[macro_use]
extern crate diesel;
use crate::model::{NewPost, Post};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::RunQueryDsl;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use schema::posts::dsl::{id, posts, published};
use std::env;
use std::io::{stdin, Read};
pub mod model;
pub mod schema;

extern crate diesel_migrations;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut pgconn = PgConnection::establish(&database_url)
        .unwrap_or_else(|err| panic!("Error connecting to {} {}", database_url, err.to_string()));

    let create_db_query = sql_query(format!("CREATE DATABASE Testing").as_str());
    match create_db_query.execute(&mut pgconn) {
        Ok(_) => {
            println!("Database testing has been created");
        }
        Err(err) => {
            println!("Can't create db {}", err.to_string());
        }
    }
    pgconn
}

fn main() {
    println!("Hello, world!");

    let connection = &mut establish_connection();

    connection.run_pending_migrations(MIGRATIONS).unwrap();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, "Fuck", "Fuck the book");
    println!("\nSaved draft {} with id {}", "Fuck", post.id);

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
        .unwrap();
    println!("Published post {}", post.title);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";
