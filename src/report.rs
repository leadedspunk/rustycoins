use crate::structs::LedgerEntry;
use crate::{schema, structs::Account};
use chrono::NaiveDate;
use diesel::query_dsl::RunQueryDsl;
use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use std::env;

const CURR: &str = "MVR";

macro_rules! load_entities {
    ($con:expr, $table:expr, $entity:ty) => {
        $table
            .load::<$entity>($con)
            .expect("Error loading entities")
    };
}

struct ReportAccount {
    id: i32,
    parent: i32,
    name: String,
    total: f32,
}

impl ReportAccount {
    fn new(a: &Account, t: f32) -> ReportAccount {
        ReportAccount{
            id: a.id.unwrap(),
            parent: a.parent,
            name: a.name.clone(),
            total: t,
        }
    }
}

pub fn generate(from: NaiveDate, to: NaiveDate) {
    let e = "";
    let ledgers_all = load_entities!(&mut establish_connection(), schema::ledger::table, LedgerEntry);
    let accounts_all = load_entities!(&mut establish_connection(), schema::accounts::table, Account);
    let mut ledgers: Vec<LedgerEntry> = vec![];
    for l in ledgers_all {
        if l.date >= from && l.date <= to {
            ledgers.push(l);
        }
    }
    println!("{:*<54}", e,);
    if from == to {
        let r_txt = format!("REPORT {}", from);
        println!("{:^54}", r_txt);
    }else {
        let r_txt = format!("REPORT {} to {}", from, to);
        println!("{:^54}", r_txt);
    }
    println!("{:*<54}", e,);

    // Root Accounts
    let mut r_accs: Vec<ReportAccount> = vec![];
    for a1 in &accounts_all {
        if a1.parent != 0 {
            // EXPENSE & ASSETS
            if a1.parent == 1 || a1.parent == 2{
                let mut a_total: f32 = 0.0;
                for l in &ledgers{
                    if l.account == a1.id.unwrap(){
                        a_total = a_total - l.credit_amount;
                        a_total = a_total + l.debit_amount;
                    }
                }
                r_accs.push(ReportAccount::new(a1, a_total))
            }
            // REVENUE & LIABILITY
            if a1.parent == 3 || a1.parent == 4{
                let mut a_total: f32 = 0.0;
                for l in &ledgers{
                    if l.account == a1.id.unwrap(){
                        a_total = a_total + l.credit_amount;
                        a_total = a_total - l.debit_amount;
                    }
                }
                r_accs.push(ReportAccount::new(a1, a_total))
            }
        }
    }
    for a1 in &accounts_all {
        if a1.parent == 0 {
            let mut total: f32 = 0.0;
            for rac in &r_accs {
                if rac.parent == a1.id.unwrap(){
                    total = total + rac.total;
                }
            }
            r_accs.push(ReportAccount::new(&a1, total));
        }

    }

    
    for a in &r_accs{
        if a.parent == 0 {
            println!("");
            println!("{:.<30}{:.>10.2} {:<5}", a.name,a.total,CURR);
            println!("");
            for a2 in &r_accs{
                if a2.parent == a.id{
                    let p = (a2.total / a.total) * 100.0;
                    println!("=>{:.<28}{:.>10.2} {:-<3}  {:<6.2} %", a2.name,a2.total, CURR, p);
                }
            }
            println!("");
            println!("{:*>54}", e);
        }
    }

}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
