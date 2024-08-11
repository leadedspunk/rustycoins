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
    let expenses:Account = Account::new("Expenses", 0);
    let assets:Account = Account::new("Assets", 0);
    let revenue:Account = Account::new("Revenue", 0);
    let liabilites:Account = Account::new("Liabilities", 0);

    let cash:Account = Account::new("Cash", 2);
    let bank:Account = Account::new("Bank", 2);

    let salary:Account = Account::new("Salary", 3);

    let mut connection = establish_connection();

    let accs = vec![expenses,assets,revenue,liabilites,cash,bank,salary];

    diesel::insert_into(schema::accounts::table)
        .values(accs)
        .execute(&mut connection)
        .expect("Error adding accounts");

}