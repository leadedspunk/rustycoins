use leptos::*;
use crate::models::{Account, LedgerEntry};

#[component]
pub fn LedgerView() -> impl IntoView {
    let (accounts, set_accounts) = create_signal(Vec::new());
    let (selected_account, set_selected_account) = create_signal(0i64);
    let (ledger_entries, set_ledger_entries) = create_signal(Vec::new());

    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(fetched_accounts) = invoke("cmd_get_accounts", &()).await {
                set_accounts.update(|a| *a = fetched_accounts);
            }
        });
    });

    create_effect(move |_| {
        let account_id = selected_account.get();
        if account_id != 0 {
            spawn_local(async move {
                if let Ok(entries) = invoke("cmd_get_ledger", &serde_json::json!({ "account_id": account_id })).await {
                    set_ledger_entries.update(|e| *e = entries);
                }
            });
        }
    });

    view! {
        <div>
            <h2>"Ledger"</h2>
            <div>
                <label>"Select Account: "
                    <select on:change=move |ev| set_selected_account.update(|a| *a = event_target_value(&ev).parse().unwrap_or(0))>
                        <option value="0" selected="selected">"Select an account"</option>
                        {move || accounts.get().into_iter().map(|account| view! {
                            <option value={account.id.to_string()}>{account.name}</option>
                        }).collect::<Vec<_>>()}
                    </select>
                </label>
            </div>
            <table>
                <thead>
                    <tr>
                        <th>"Date"</th>
                        <th>"Description"</th>
                        <th>"Debit"</th>
                        <th>"Credit"</th>
                        <th>"Balance"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || ledger_entries.get().into_iter().map(|entry| view! {
                        <tr>
                            <td>{entry.date}</td>
                            <td>{entry.description}</td>
                            <td>{entry.debit_amount}</td>
                            <td>{entry.credit_amount}</td>
                            <td>{entry.balance}</td>
                        </tr>
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}