#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
mod db;
mod models;
mod handlers;

use crate::handlers::{create_account, create_transaction, delete_transaction, get_accounts, get_transactions, get_ledger};
use crate::models::{Account, Transaction, LedgerEntry};
use rust_decimal::Decimal;
use chrono::NaiveDate;

#[tauri::command]
async fn cmd_create_account(state: tauri::State<'_, db::Pool>, name: String, account_type: String) -> Result<Account, String> {
    create_account(&state, name, account_type).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_create_transaction(
    state: tauri::State<'_, db::Pool>,
    date: String,
    description: String,
    debit_account_id: i64,
    credit_account_id: i64,
    amount: String,
) -> Result<(), String> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| e.to_string())?;
    let amount = Decimal::from_str_exact(&amount).map_err(|e| e.to_string())?;
    create_transaction(&state, date, description, debit_account_id, credit_account_id, amount)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_delete_transaction(state: tauri::State<'_, db::Pool>, transaction_id: i64) -> Result<(), String> {
    delete_transaction(&state, transaction_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_get_accounts(state: tauri::State<'_, db::Pool>) -> Result<Vec<Account>, String> {
    get_accounts(&state).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_get_transactions(state: tauri::State<'_, db::Pool>) -> Result<Vec<Transaction>, String> {
    get_transactions(&state).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_get_ledger(state: tauri::State<'_, db::Pool>, account_id: i64) -> Result<Vec<LedgerEntry>, String> {
    get_ledger(&state, account_id).await.map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            tauri::async_runtime::block_on(async move {
                let pool = db::establish_connection().await.expect("Failed to connect to database");
                handle.manage(pool);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd_create_account,
            cmd_create_transaction,
            cmd_delete_transaction,
            cmd_get_accounts,
            cmd_get_transactions,
            cmd_get_ledger,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}