use crate::models::{Account, Transaction, Entry, LedgerEntry};
use sqlx::{Pool, Sqlite};
use rust_decimal::Decimal;
use chrono::NaiveDate;

pub async fn create_account(pool: &Pool<Sqlite>, name: String, account_type: String) -> Result<Account, sqlx::Error> {
    let account = sqlx::query_as!(
        Account,
        "INSERT INTO accounts (name, account_type) VALUES (?, ?) RETURNING *",
        name,
        account_type
    )
    .fetch_one(pool)
    .await?;

    Ok(account)
}

pub async fn create_transaction(
    pool: &Pool<Sqlite>,
    date: NaiveDate,
    description: String,
    debit_account_id: i64,
    credit_account_id: i64,
    amount: Decimal,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    let transaction = sqlx::query_as!(
        Transaction,
        "INSERT INTO transactions (date, description) VALUES (?, ?) RETURNING *",
        date,
        description
    )
    .fetch_one(&mut tx)
    .await?;

    // Create debit entry
    sqlx::query!(
        "INSERT INTO entries (transaction_id, account_id, amount, entry_type) VALUES (?, ?, ?, ?)",
        transaction.id,
        debit_account_id,
        amount,
        "DEBIT"
    )
    .execute(&mut tx)
    .await?;

    // Create credit entry
    sqlx::query!(
        "INSERT INTO entries (transaction_id, account_id, amount, entry_type) VALUES (?, ?, ?, ?)",
        transaction.id,
        credit_account_id,
        amount,
        "CREDIT"
    )
    .execute(&mut tx)
    .await?;

    // Update ledger for debit account
    update_ledger(&mut tx, debit_account_id, transaction.id, date, &description, amount, Decimal::ZERO).await?;

    // Update ledger for credit account
    update_ledger(&mut tx, credit_account_id, transaction.id, date, &description, Decimal::ZERO, amount).await?;

    tx.commit().await?;

    Ok(())
}

async fn update_ledger(
    tx: &mut sqlx::Transaction<'_, Sqlite>,
    account_id: i64,
    transaction_id: i64,
    date: NaiveDate,
    description: &str,
    debit_amount: Decimal,
    credit_amount: Decimal,
) -> Result<(), sqlx::Error> {
    let last_balance = sqlx::query_scalar!(
        "SELECT balance FROM ledger WHERE account_id = ? ORDER BY id DESC LIMIT 1",
        account_id
    )
    .fetch_optional(tx)
    .await?
    .unwrap_or(Decimal::ZERO);

    let new_balance = last_balance + debit_amount - credit_amount;

    sqlx::query!(
        "INSERT INTO ledger (account_id, transaction_id, date, description, debit_amount, credit_amount, balance) VALUES (?, ?, ?, ?, ?, ?, ?)",
        account_id,
        transaction_id,
        date,
        description,
        debit_amount,
        credit_amount,
        new_balance
    )
    .execute(tx)
    .await?;

    Ok(())
}

pub async fn delete_transaction(pool: &Pool<Sqlite>, transaction_id: i64) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Delete entries
    sqlx::query!("DELETE FROM entries WHERE transaction_id = ?", transaction_id)
        .execute(&mut tx)
        .await?;

    // Delete ledger entries
    sqlx::query!("DELETE FROM ledger WHERE transaction_id = ?", transaction_id)
        .execute(&mut tx)
        .await?;

    // Delete transaction
    sqlx::query!("DELETE FROM transactions WHERE id = ?", transaction_id)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn get_accounts(pool: &Pool<Sqlite>) -> Result<Vec<Account>, sqlx::Error> {
    let accounts = sqlx::query_as!(Account, "SELECT * FROM accounts")
        .fetch_all(pool)
        .await?;

    Ok(accounts)
}

pub async fn get_transactions(pool: &Pool<Sqlite>) -> Result<Vec<Transaction>, sqlx::Error> {
    let transactions = sqlx::query_as!(Transaction, "SELECT * FROM transactions")
        .fetch_all(pool)
        .await?;

    Ok(transactions)
}

pub async fn get_ledger(pool: &Pool<Sqlite>, account_id: i64) -> Result<Vec<LedgerEntry>, sqlx::Error> {
    let ledger_entries = sqlx::query_as!(
        LedgerEntry,
        "SELECT * FROM ledger WHERE account_id = ? ORDER BY date",
        account_id
    )
    .fetch_all(pool)
    .await?;

    Ok(ledger_entries)
}