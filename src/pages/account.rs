use dioxus::prelude::*;
use serde::Deserialize;
use crate::{CLIENT, TOKEN, BASE_URL};

#[derive(Deserialize)]
struct Account {
    pub id: String,
    pub name: String,
    pub todo_count: u64
}

pub fn Account(cx: Scope) -> Element {
    let client = use_read(&cx, CLIENT);
    let user_token = use_read(&cx, TOKEN);

    let router = use_router(&cx);
    let account_state = use_state(&cx, || None::<Account>);

    cx.render(match user_token.as_ref().cloned() {
        None => {
            router.push_route("/login", None, None);
            rsx!{ div { "Rerouting" }}
        },
        Some(token) => {
            let client = client.clone();

            cx.use_hook(|_| cx.spawn({
                let account_state = account_state.clone();

                async move {
                    let account = client
                        .get(format!("{BASE_URL}/api/accounts/@me"))
                        .header("Authorization", token)
                        .send()
                        .await
                        .unwrap()
                        .json::<Account>()
                        .await
                        .unwrap();

                    account_state.set(Some(account));
                }
            }));

            rsx!{
                div {
                    if let Some(account) = account_state.get() {
                        rsx! {
                            div {
                                h1 {
                                    "{account.name}"
                                },
                                p {
                                    "You have {account.todo_count} todos"
                                }
                            }
                        }
                    } else {
                        rsx! {
                            None::<()>
                        }
                    }
                }
            }
        }
    })
}
