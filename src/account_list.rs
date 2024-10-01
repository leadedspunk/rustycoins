use leptos::*;
use crate::models::Account;

#[component]
pub fn AccountList() -> impl IntoView {
    let (accounts, set_accounts) = create_signal(Vec::new());

    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(fetched_accounts) = invoke("cmd_get_accounts", &()).await {
                set_accounts.update(|a| *a = fetched_accounts);
            }
        });
    });

    view! {
        <div>
            <h2>"Accounts"</h2>
            <ul>
                {move || accounts.get().into_iter().map(|account| view! {
                    <li>{account.name} " (" {account.account_type} ")"</li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}