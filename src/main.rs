use chrono;
use chrono::NaiveDate;
use clap::{command, Parser, Subcommand};
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use dotenv::dotenv;
use rustycoins::bootstrap_database;
use rustycoins::schema;
use rustycoins::schema::accounts;
use rustycoins::schema::journal;
use rustycoins::schema::ledger;
use rustycoins::schema::transactions;
use rustycoins::structs::{Account, JournalEntry, LedgerEntry, MTransaction};
use std::env;
use std::path::Path;
use csv::ReaderBuilder;
use rustycoins::report;

macro_rules! load_entities {
    ($con:expr, $table:expr, $entity:ty) => {
        $table
            .load::<$entity>($con)
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

pub fn insert_transaction(new_transaction: MTransaction){
    let mut con = establish_connection();
    diesel::insert_into(transactions::table)
                    .values(new_transaction)
                    .execute(&mut con)
                    .expect("Error creating account");

                use schema::transactions::dsl::id;
                let mut new_transaction: MTransaction =
                    schema::transactions::table
                    .order(id.desc())
                    .first(&mut con).unwrap();

                let (ledgers, journals) = new_transaction.create_entrys();

                diesel::insert_into(ledger::table)
                    .values(ledgers)
                    .execute(&mut con)
                    .expect("Error creating account");

                diesel::insert_into(journal::table)
                    .values(journals)
                    .execute(&mut con)
                    .expect("Error creating account");
}


pub fn get_account_name(con: &mut SqliteConnection, acct_id: i32) -> String {
    use crate::schema::accounts::dsl::{accounts, id, name};

    let accoount_name = accounts
        .filter(id.eq(acct_id))
        .select(name)
        .first(con)
        .unwrap();

    accoount_name
}

pub fn get_ledger_entries(con: &mut SqliteConnection, acct_id: i32) -> Vec<LedgerEntry> {
    use crate::schema::ledger::dsl::{account, ledger};
    if acct_id != 0 {
        let ledgers = ledger
        .filter(account.eq(acct_id))
        .load::<LedgerEntry>(con)
        .unwrap();

        ledgers
    }else {
        let ledgers = load_entities!(con, schema::ledger::table, LedgerEntry);
        ledgers
    }
    
}

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
    Ledger {
        #[command(subcommand)]
        action: LedgerAction,
    },
    Journal {
        #[command(subcommand)]
        action: JournalAction,
    },
    Report {
        #[command(subcommand)]
        action: ReportAction,
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
    Drop { id: i32 },
    /// List all accounts
    List,
}

#[derive(Subcommand, Debug)]
enum TransactionAction {
    // Create a new transaction
    New {
        // Transaction amount
        amount: f32,
        //Transaction Date and time
        datetime: String,
        // Account to be credited
        credit: i32,
        // Account to be debited
        debit: i32,
        // Description
        desc: String,
    },
    Import {
        file: String,
    },
    // List all transactions
    List,
}

#[derive(Subcommand, Debug)]
enum LedgerAction {
    // List all transactions
    List { account_id: i32 },
}

#[derive(Subcommand, Debug)]
enum JournalAction {
    // List all transactions
    List,
}

#[derive(Subcommand, Debug)]
enum ReportAction {
    // List all transactions
    Generate {
        date_1: NaiveDate,
        date_2: Option<NaiveDate>,
    },
}

fn main() {
    env::set_var("DATABASE_URL", "./db.sqlite");
    let mut con = establish_connection();
    let args = Args::parse();

    match args.command {
        Commands::Init => {
            println!("Initializing application...");
            bootstrap_database::start();
            // Initialize the database or perform other setup tasks
        }

        Commands::Account { action } => match action {
            AccountAction::New { name, parent } => {
                println!("Creating a new account with name: {}", name);
                // Create a new account with the given name
                let new_account = Account::new(name.as_str(), parent);
                diesel::insert_into(accounts::table)
                    .values(new_account)
                    .execute(&mut con)
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
                let accounts: Vec<Account> =
                    load_entities!(&mut con, schema::accounts::table, Account);
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
                datetime,
                desc,
            } => {
                let ddate: NaiveDate =
                    NaiveDate::parse_from_str(&datetime, "%Y-%m-%d").expect("Invalid Date Format");
                println!("Creating a new transaction:");
                println!("  Amount: {}", amount);
                println!("  Credit: {}", credit);
                println!("  Debit: {}", debit);

                // Create a new transaction with the given details
                let mut new_transaction: MTransaction =
                    MTransaction::new(ddate, amount, credit, debit);
                new_transaction.description(&desc);

                insert_transaction(new_transaction);
            }
            TransactionAction::List => {
                let transactionlist =
                    load_entities!(&mut con, schema::transactions::table, MTransaction);
                println!(
                    "{:<5}|{:<10}|{:<10}|{:<8}|{:<8}|{:<30}",
                    "ID", "Date", "Amount", "Credit", "Debit", "Description"
                );
                for item in transactionlist {
                    println!(
                        "{:<5}|{:<10}|{:<10}|{:<8}|{:<8}|{:<30}",
                        item.id.unwrap(),
                        item.date,
                        item.amount,
                        item.credit_account,
                        item.debit_account,
                        item.description.unwrap()
                    );
                }
            }
            TransactionAction::Import {file } => {
                let mut reader = ReaderBuilder::new()
                .has_headers(true)
                .from_path(file)
                .expect("error loading file");

                for result in reader.deserialize() {
                    let record: MTransaction  = result.expect("Error");
                    insert_transaction(record);
                }
            }
        },
        Commands::Ledger { action } => match action {
            LedgerAction::List { account_id } => {
                let ledgerlist = get_ledger_entries(&mut con, account_id);
                println!(
                    "{:<5}|{:<10}|{:<15}|{:<10}|{:<10}|{:<30}",
                    "ID", "Date", "Account", "Credit", "Debit", "Description"
                );
                for item in ledgerlist {
                    println!(
                        "{:<5}|{:<10}|{:<15}|{:<10}|{:<10}|{:<30}",
                        item.id.unwrap(),
                        item.date,
                        get_account_name(&mut con, item.account),
                        item.credit_amount,
                        item.debit_amount,
                        item.description.unwrap()
                    );
                }
            }
        },
        Commands::Journal { action } => match action {
            JournalAction::List => {
                let journallist = load_entities!(&mut con, schema::journal::table, JournalEntry);
                println!(
                    "{:<5}|{:<10}|{:<10}|{:<10}",
                    "ID", "Account", "Credit", "Debit"
                );
                for item in journallist {
                    println!(
                        "{:<5}|{:<10}|{:<10}|{:<10}",
                        item.id.unwrap(),
                        get_account_name(&mut con, item.account),
                        item.credit_amount,
                        item.debit_amount
                    );
                }
            }
        },
        Commands::Report { action } => match action {
            ReportAction::Generate { date_1, date_2 } => {
                if let Some(date_2) = date_2 {
                    report::generate(date_1, date_2);
                }else {
                    report::generate(date_1, date_1);
                }
                // report::generate();
            }
        },
    }
}
