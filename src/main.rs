use clap::{command, Parser, Subcommand};
use diesel::query_dsl::methods::FilterDsl;
use diesel::{Connection, ExpressionMethods, RunQueryDsl, SqliteConnection};
use dotenv::dotenv;
use rustycoins::bootstrap_database;
use rustycoins::schema;
use rustycoins::schema::accounts;
use rustycoins::structs::{Account, MTransaction};
use std::any::Any;
use std::env;
use std::path::Path;
use time::OffsetDateTime;

macro_rules! load_entities {
    ($table:expr, $entity:ty) => {
        $table
            .load::<$entity>(&mut establish_connection())
            .expect("Error loading entities")
    };
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    if !Path::new(&database_url).exists() {
        bootstrap_database::start();
    }
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// let accounts: Vec<Account> = load_entities!(schema::accounts::table, Account);
//     println!("{:<3}|{:<20}|{:<3}", "ID", "Name", "Parent");
//     for acc in accounts{
//         println!("{:<3}|{:<20}|{:<3}", acc.id.unwrap(), acc.name, acc.parent);
//     }

/// RustyCoins: A Rust-based personal financing tool
#[derive(Parser, Debug)]
#[command(version = "1.0", author = "Author Name <ravenistaken@proton.me>")]
struct Args {
    /// The subcommand to run
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize the application
    Init,
    /// Configure accounts
    Account {
        #[command(subcommand)]
        action: AccountAction,
    },
    /// Configure transactions
    Transaction {
        #[command(subcommand)]
        action: TransactionAction,
    },
}

#[derive(Subcommand, Debug)]
enum AccountAction {
    /// Create a new account
    New {
        /// Name of the account
        name: String,
        /// Parent account
        parent: i32,
    },
    /// Delete an account
    Drop {
        id: i32,
    },
    /// List all accounts
    List,
}

#[derive(Subcommand, Debug)]
enum TransactionAction {
    /// Create a new transaction
    New {
        /// Transaction amount
        amount: f32,
        /// Account to be credited
        credit: i32,
        /// Account to be debited
        debit: i32,
    },
}

fn main() {
    env::set_var("DATABASE_URL", "./db.sqlite");

    let mut con = establish_connection();
    let accounts = schema::accounts::table
        .load::<Account>(&mut con)
        .expect("Error loading users");

    let args = Args::parse();

    match args.command {
        Commands::Init => {
            println!("Initializing application...");
            bootstrap_database::start();
            // Initialize the database or perform other setup tasks
        }

        Commands::Account { action } => match action {
            AccountAction::New { name, parent} => {
                println!("Creating a new account with name: {}", name);
                // Create a new account with the given name
                let new_account = Account::new(name.as_str(), parent);
                diesel::insert_into(accounts::table)
                .values(new_account)
                .execute(&mut establish_connection())
                .expect("Error creating account");
            }
            AccountAction::Drop { id } => {
                println!("Dropping Accout {}", id);
                let target = accounts::dsl::accounts.filter(accounts::dsl::id.eq(id));
                let _ = diesel::delete(target).execute(&mut con);
            }
            AccountAction::List => {
                println!("Listing all accounts...");
                // List all accounts
                let accounts: Vec<Account> = load_entities!(schema::accounts::table, Account);
                println!("{:<3}|{:<20}", "ID", "Name");
                for acc in &accounts {
                    if acc.parent == 0 {
                        let a_id = acc.id.unwrap();
                        println!("{:<3}|{:<20}", &a_id, acc.name);
                        for subacc in &accounts {
                            if subacc.parent == a_id {
                                let a2_id = subacc.id.unwrap();
                                let name = format!("↳ {}", subacc.name);
                                println!("{:<3}|{:<20}", a2_id, name);
                                for subacc2 in &accounts {
                                    if subacc2.parent == a2_id {
                                        let name2 = format!("  ↳ {}", subacc2.name);
                                        println!("{:<3}|{:<30}", subacc2.id.unwrap(), name2);
                                        
                                    }
                                }
                                
                            }
                        }
                    }
                    
                }
            }
        },

        Commands::Transaction { action } => match action {
            TransactionAction::New {
                amount,
                credit,
                debit,
            } => {
                println!("Creating a new transaction:");
                println!("  Amount: {}", amount);
                println!("  Credit: {}", credit);
                println!("  Debit: {}", debit);
                // Create a new transaction with the given details
                let _new_transaction = MTransaction::new(OffsetDateTime::now_utc(), amount, credit, debit);
            }
        },
    }
}
