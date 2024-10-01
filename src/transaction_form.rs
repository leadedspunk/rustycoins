use leptos::*;
use crate::models::Account;

#[component]
pub fn TransactionForm() -> impl IntoView {
    let (accounts, set_accounts) = create_signal(Vec::new());
    let (date, set_date) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (debit_account, set_debit_account) = create_signal(0i64);
    let (credit_account, set_credit_account) = create_signal(0i64);
    let (amount, set_amount) = create_signal(String::new());

    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(fetched_accounts) = invoke("cmd_get_accounts", &()).await {
                set_accounts.update(|a| *a = fetched_accounts);
            }
        });
    });

    let submit = create_action(move |_| async move {
        let result = invoke(
            "cmd_create_transaction",
            &serde_json::json!({
                "date": date.get(),
                "description": description.get(),
                "debit_account_id": debit_account.get(),
                "credit_account_id": credit_account.get(),
                "amount": amount.get(),
            }),
        )
        .await;

        match result {
            Ok(_) => {
                // Clear form
                set_date.update(|d| *d = String::new());
                set_description.update(|d| *d = String::new());
                set_debit_account.update(|a| *a = 0);
                set_credit_account.update(|a| *a = 0);
                set_amount.update(|a| *a = String::new());
            }
            Err(e) => {
                // Handle error (e.g., show error message)
                log!("Error creating transaction: {:?}", e);
            }
        }
    });

    view! {
        <form on:submit=move |ev| { ev.prevent_default(); submit.dispatch(()) }>
            <h2>"New Transaction"</h2>
            <div>
                <label>"Date: "
                    <input
                        type="date"
                        on:input=move |ev| set_date.update(|d| *d = event_target_value(&ev))
                        prop:value=date
                    />
                </label>
            </div>
            <div>
                <label>"Description: "
                    <input
                        type="text"
                        on:input=move |ev| set_description.update(|d| *d = event_target_value(&ev))
                        prop:value=description
                    />
                </label>
            </div>
            <div>
                <label>"Debit Account: "
                    <select on:change=move |ev| set_debit_account.update(|a| *a = event_target_value(&ev).parse().unwrap_or(0))>
                        <option value="0" selected="selected">"Select an account"</option>
                        {move || accounts.get().into_iter().map(|account| view! {
                            <option value={account.id.to_string()}>{account.name}</option>
                        }).collect::<Vec<_>>()}
                    </select>
                </label>
            </div>
            <div>
                <label>"Credit Account: "
                    <select on:change=move |ev| set_credit_account.update(|a| *a = event_target_value(&ev).parse().unwrap_or(0))>
                        <option value="0" selected="selected">"Select an account"</option>
                        {move || accounts.get().into_iter().map(|account| view! {
                            <option value={account.id.to_string()}>{account.name}</option>
                        }).collect::<Vec<_>>()}
                    </select>
                </label>
            </div>
            <div>
                <label>"Amount: "
                    <input
                        type="text"
                        on:input=move |ev| set_amount.update(|a| *a = event_target_value(&ev))
                        prop:value=amount
                    />
                </label>
            </div>
            <button type="submit">"Create Transaction"</button>
        </form>
    }
}