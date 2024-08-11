use diesel::prelude::*;
use crate::schema::{accounts, journal, ledger, transactions};
use chrono::{self, NaiveDate};

#[derive(Queryable, Selectable, Insertable)]
#[table_name = "accounts"]
pub struct Account{
    pub id: Option<i32>,
    pub name: String,
    pub parent: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[table_name = "transactions"]
pub struct MTransaction{
    pub id: Option<i32>,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub amount: f32,
    pub credit_account: i32,
    pub debit_account: i32,
}

#[derive(Queryable, Selectable, Insertable)]
#[table_name = "journal"]
pub struct JournalEntry{
    pub id:Option<i32>,
    pub transaction: i32,
    pub account: i32,
    pub debit_amount: f32,
    pub credit_amount: f32,
}

#[derive(Queryable, Selectable, Insertable)]
#[table_name = "ledger"]
pub struct LedgerEntry{
    pub id: Option<i32>,
    pub account: i32,
    pub transaction: i32,
    pub date: NaiveDate,
    pub description: Option<String>,
    pub credit_amount: f32,
    pub debit_amount: f32,
    pub balance: f32,
}

impl Account {
    pub fn new(name: &str, parent: i32) -> Self {
        Account{
            id: None,
            name: name.to_string(),
            parent,
        }
    }
}

impl MTransaction {
    pub fn new(date: NaiveDate, amount: f32, credit_account: i32, debit_account: i32) -> Self {
        MTransaction {
            id: None,
            date,
            description: Some("".to_string()),
            amount,
            credit_account,
            debit_account,
        }
    }

    pub fn description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }

    pub fn create_entrys(&mut self) -> (Vec<LedgerEntry>, Vec<JournalEntry>){
        let desc: String = self.description.clone().unwrap().as_str().to_string();
        let lcredit: LedgerEntry = LedgerEntry::new_credit(self.id.unwrap(), self.credit_account, self.date, desc, self.amount);
        let desc: String = self.description.clone().unwrap().as_str().to_string();
        let ldebit: LedgerEntry = LedgerEntry::new_debit(self.id.unwrap(), self.debit_account, self.date, desc, self.amount);

        let jcredit: JournalEntry = JournalEntry::new_credit(self.id.unwrap(), self.credit_account, self.amount);
        let jdebit: JournalEntry = JournalEntry::new_debit(self.id.unwrap(), self.debit_account, self.amount);

        let ledgers: Vec<LedgerEntry> = vec![lcredit,ldebit];
        let journals: Vec<JournalEntry> = vec![jcredit,jdebit];

        (ledgers, journals)
    }
}

impl JournalEntry {
    fn new_credit(transaction: i32, account_id: i32, amount: f32) -> Self {
        JournalEntry {
            id: None,
            transaction,
            account: account_id,
            debit_amount: 0.0,
            credit_amount: amount,
        }
    }

    fn new_debit(transaction: i32, account_id: i32, amount: f32) -> Self {
        JournalEntry {
            id: None,
            transaction,
            account: account_id,
            debit_amount: amount,
            credit_amount: 0.0,
        }
    }
}

impl LedgerEntry {
    fn new_credit(transaction: i32, account_id: i32, date: NaiveDate, description: String, amount: f32) -> Self {
        LedgerEntry {
            id: None,
            account: account_id,
            transaction,
            date,
            description: Some(description),
            credit_amount: amount,
            debit_amount: 0.0,
            balance: 0.0,
        }
    }

    fn new_debit(transaction: i32, account_id: i32, date: NaiveDate, description: String, amount: f32) -> Self {
        LedgerEntry {
            id: None,
            account: account_id,
            transaction,
            date,
            description: Some(description),
            credit_amount: 0.0,
            debit_amount: amount,
            balance: 0.0,
        }
    }

    pub fn balance_update(&mut self, balance: f32){
        self.balance = balance;
    }
}