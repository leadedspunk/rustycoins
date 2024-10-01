use leptos::*;
use crate::models::Transaction;

#[component]
pub fn TransactionList() -> impl IntoView {
    let (transactions, set_transactions) = create_signal(Vec::new());

    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(fetched_transactions) = invoke("cmd_get_transactions", &()).await {
                set_transactions.update(|t| *t = fetched_transactions);
            }
        });
    });

    let delete_transaction = create_action(move |transaction_id: &i64| {
        let id = *transaction_id;
        async move {
            if let Ok(_) = invoke("cmd_delete_transaction", &serde_json::json!({ "transaction_id": id })).await {
                // Refresh the transaction list
                if let Ok(fetched_transactions) = invoke("cmd_get_transactions", &()).await {
                    set_transactions.update(|t| *t = fetched_transactions);
                }
            }
        }
    });

    view! {
        <div>
            <h2>"Transactions"</h2>
            <table>
                <thead>
                    <tr>
                        <th>"Date"</th>
                        <th>"Description"</th>
                        <th>"Actions"</th>
                    </tr>
                </thead>
                <tbody>
                    {move || transactions.get().into_iter().map(|transaction| view! {
                        <tr>
                            <td>{transaction.date}</td>
                            <td>{transaction.description}</td>
                            <td>
                                <button on:click=move |_| delete_transaction.dispatch(transaction.id)>
                                    "Delete"
                                </button>
                            </td>
                        </tr>
                    }).collect::<Vec<_>>()}
                </tbody>
            </table>
        </div>
    }
}