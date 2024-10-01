use leptos::*;
use crate::components::{AccountList, TransactionForm, TransactionList, LedgerView};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <h1>"Bookkeeper"</h1>
            <div style="display: flex; justify-content: space-between;">
                <div style="width: 30%;">
                    <AccountList />
                </div>
                <div style="width: 65%;">
                    <TransactionForm />
                    <TransactionList />
                    <LedgerView />
                </div>
            </div>
        </main>
    }
}