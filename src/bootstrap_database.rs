use crate::{schema, structs::Account};
use diesel::{self, connection, Connection, RunQueryDsl, SqliteConnection};
use serde;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn start() {
    let expenses:Account = Account::new("Expenses", 0);
    let assets:Account = Account::new("Assets", 0);
    let revenue:Account = Account::new("Revenue", 0);
    let liabilites:Account = Account::new("Liabilities", 0);

    let mut connection = establish_connection();

    diesel::insert_into(schema::accounts::table)
        .values(expenses)
        .execute(&mut connection)
        .expect("Error adding accounts");

    diesel::insert_into(schema::accounts::table)
        .values(assets)
        .execute(&mut connection)
        .expect("Error adding accounts");

    diesel::insert_into(schema::accounts::table)
        .values(revenue)
        .execute(&mut connection)
        .expect("Error adding accounts");

    diesel::insert_into(schema::accounts::table)
        .values(liabilites)
        .execute(&mut connection)
        .expect("Error adding accounts");


    let mut connection2 = establish_connection();

    let results = schema::accounts::table
        .load::<Account>(&mut connection2)
        .expect("Error loading users");

    for acc in results{
        println!("ID: {}, Name: {}, Email: {}", acc.id.unwrap(), acc.name, acc.parent);
    }
}