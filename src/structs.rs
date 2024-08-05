use time::{self, OffsetDateTime};
use rand::random;

pub struct Account{
    pub id: u16,
    name: String,
    parent: u16,
}

pub struct MTransaction{
    pub id: u32,
    date: time::Date,
    time: time::Time,
    description: String,
    amount: f32,
    credit_account: u16,
    debit_account: u16,
}

// pub struct JournalEntry{
//     pub id: u32,
//     transaction: u32,
//     account: u16,
//     debit_amount: f32,
//     credit_amount: f32,
// }

pub struct LedgerEntry{
    pub id: u32,
    transaction: u32,
    date: time::Date,
    time: time::Time,
    description: String,
    credit_amount: f32,
    debit_amount: f32,
    balance: f32,
}

impl Account {
    pub fn new(name: &str, parent: u16) -> Self {
        Account{
            id: random(),
            name: name.to_string(),
            parent,
        }
    }
}

impl MTransaction {
    pub fn new(date_time: OffsetDateTime, amount: f32, credit_account: u16, debit_account: u16) -> Self {
        let date = date_time.date();
        let time = date_time.time();
        MTransaction {
            id: random(),
            date,
            time,
            description: "".to_string(),
            amount,
            credit_account,
            debit_account,
        }
    }

    pub fn description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn create_entrys(&mut self) -> (LedgerEntry, LedgerEntry){
        let desc = self.description.as_str().to_string();
        let credit = LedgerEntry::new_credit(self.id, self.date, self.time, desc, self.amount);
        let desc = self.description.as_str().to_string();
        let debit = LedgerEntry::new_debit(self.id, self.date, self.time, desc, self.amount);

        (credit, debit)
    }
}

// impl JournalEntry {
//     fn new_credit(transaction: u32, account: u16, ledger_entry: LedgerEntry, amount: f32) -> Self {
//         JournalEntry {
//             id: random(),
//             transaction,
//             account,
//             debit_amount: 0.0,
//             credit_amount: amount,
//         }
//     }

//     fn new_debit(transaction: u32, account: u16, ledger_entry: LedgerEntry, amount: f32) -> Self {
//         JournalEntry {
//             id: random(),
//             transaction,
//             account,
//             debit_amount: amount,
//             credit_amount: 0.0,
//         }
//     }
// }

impl LedgerEntry {
    fn new_credit(transaction: u32, date: time::Date, time: time::Time, description: String, amount: f32) -> Self {
        LedgerEntry {
            id: random(),
            transaction,
            date,
            time,
            description,
            credit_amount: amount,
            debit_amount: 0.0,
            balance: 0.0,
        }
    }

    fn new_debit(transaction: u32, date: time::Date, time: time::Time, description: String, amount: f32) -> Self {
        LedgerEntry {
            id: random(),
            transaction,
            date,
            time,
            description,
            credit_amount: 0.0,
            debit_amount: amount,
            balance: 0.0,
        }
    }

    pub fn balance_update(&mut self, balance: f32){
        self.balance == balance;
    }
}