use crate::{schema, structs::Account};
use diesel::{self, Connection, RunQueryDsl, SqliteConnection};
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn start() {
    let mut accs = vec![];

    accs.push(Account::new("Expenses", 0));
    accs.push(Account::new("Assets", 0));
    accs.push(Account::new("Income", 0));
    accs.push(Account::new("Liabilities", 0));

    accs.push(Account::new("Other", 1));
    accs.push(Account::new("Cash", 2));
    accs.push(Account::new("Accounts Recievable", 2));
    accs.push(Account::new("Salary", 3));
    accs.push(Account::new("Accounts Payable", 4));


    diesel::insert_into(schema::accounts::table)
        .values(accs)
        .execute(&mut establish_connection())
        .expect("Error adding accounts");

}