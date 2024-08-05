use time;
use rustycoins::structs::{Account, MTransaction};

fn main() {
    //let expenses:Account = Account::new("Expenses", 0);
    let assets:Account = Account::new("Assets", 0);
    let revenue:Account = Account::new("Revenue", 0);

    let mut transaction:MTransaction = MTransaction::new(
        time::OffsetDateTime::now_utc(), 120.00,
        assets.id, revenue.id,
    );

    let (ledger_credit, ledger_debit) = transaction.create_entrys();

}