use diesel::{Connection, SqliteConnection};
use time;
use rustycoins::structs::{Account, MTransaction};
use rustycoins::bootstrap_database;
use std::path::Path;
use std::{env, fs};
use dotenv::dotenv;

fn get_accounts() {

}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
    }

fn main() {
    env::set_var("DATABASE_URL", "./db.sqlite");

    let con = establish_connection();

    bootstrap_database::start();

    println!("Hello, world!");

}