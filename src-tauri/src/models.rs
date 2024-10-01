use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub account_type: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: i64,
    pub date: NaiveDate,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Entry {
    pub id: i64,
    pub transaction_id: i64,
    pub account_id: i64,
    pub amount: Decimal,
    pub entry_type: String, // "DEBIT" or "CREDIT"
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct LedgerEntry {
    pub id: i64,
    pub account_id: i64,
    pub transaction_id: i64,
    pub date: NaiveDate,
    pub description: String,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub balance: Decimal,
}