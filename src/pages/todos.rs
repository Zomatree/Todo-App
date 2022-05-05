use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use eos::{DateTime, Utc, format_dt};
use futures::StreamExt;

use crate::{CLIENT, BASE_URL, TOKEN};

#[derive(Debug, Deserialize, Clone)]
struct Todo {
    pub title: String,
    pub todo_id: u64,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TodosResponse {
    pub todo: Vec<Todo>
}

#[derive(Debug, Serialize)]
struct CreateTodosRequest {
    pub title: String
}

enum TodoAction {
    GetAll,
    Edit(u64),
    Delete(u64),
    Create(String),
}

// i can clone the token around because it wont change between renders, the user needs to login which involves going to a diff
// route which means the hook here will rerun with the token not causing invalid state

pub fn Todos(cx: Scope) -> Element {
    let client = use_read(&cx, CLIENT);
    let user_token = use_read(&cx, TOKEN);

    let router = use_router(&cx);
    let todos_state = use_state(&cx, Vec::<Todo>::new);

    let coro_handler = use_coroutine::<TodoAction, _, _>(&cx, |mut rx| {
        let user_token = user_token.clone();
        let client = client.clone();
        let todos_state = todos_state.clone();

        async move {
            while let Some(message) = rx.next().await && let Some(ref token) = user_token {
                match message {
                    TodoAction::GetAll => {
                        let todos = client
                            .get(format!("{BASE_URL}/api/todos"))
                            .header("Authorization", token)
                            .send()
                            .await
                            .unwrap()
                            .json::<TodosResponse>()
                            .await
                            .unwrap();

                        todos_state.set(todos.todo);
                    },
                    TodoAction::Edit(todo_id) => {},
                    TodoAction::Delete(todo_id) => {
                        let response = client.delete(format!("{BASE_URL}/api/todos/{todo_id}"))
                            .header("Authorization", token)
                            .send()
                            .await
                            .unwrap();

                        if response.status() == StatusCode::NO_CONTENT {
                            todos_state.modify(|todos| {
                                let todos = todos
                                    .iter()
                                    .cloned()
                                    .filter(|todo| todo.todo_id != todo_id)
                                    .collect::<Vec<_>>();
                                log::info!("{todos:?}");
                                todos
                            })
                        }
                    },
                    TodoAction::Create(title) => {
                            let todo = client
                            .post(format!("{BASE_URL}/api/todos"))
                            .header("Authorization", token)
                            .json(&CreateTodosRequest { title })
                            .send()
                            .await
                            .unwrap()
                            .json::<Todo>()
                            .await
                            .unwrap();

                        todos_state.with_mut(|todos| todos.push(todo));
                    }
                }
            }
        }
    });

    if user_token.is_some() {
        cx.use_hook(|_| coro_handler.send(TodoAction::GetAll));

        rsx!(cx, div {
            class: "px-1",
            div {
                class: "d-flex px-2 my-2",
                button {
                    class: "d-flex btn btn-primary",
                    span { "+ New" },
                }
            }
            ol {
                class: "list-group list-group-numbered",
                todos_state.get().iter().map(|todo| {
                    let Todo { title, todo_id, completed, created_at, description } = todo;
                    let formatted_date = format_dt!("%H:%M - %d/%m/%Y", created_at);

                    rsx! {
                        div {
                            class: "list-group-item",
                            key: "{todo_id}",

                            div {
                                class: "d-flex flex-row justify-content-start align-items-center",
                                input {
                                    class: "form-check-input p-2",
                                    r#type: "checkbox",
                                    checked: "{completed}"
                                },
                                div {
                                    class: "d-flex flex-column p-2",
                                    h5 { class: "", "{title}" },
                                    p { class: "", "{formatted_date}" },
                                    description
                                        .as_ref()
                                        .map(|description| rsx! {
                                            p {
                                                class: "",
                                                "{description}"
                                            }
                                        })
                                },
                                button {
                                    class: "d-flex btn btn-danger p-2 ms-auto",
                                    onclick: |_| {
                                        coro_handler.send(TodoAction::Delete(*todo_id))
                                    },
                                    span { "delete" },
                                }
                            },
                        }
                    }
                })
            }
        })
    } else {
        router.push_route("/login", Some("Login".to_string()), None);
        rsx!(cx, div { "Rerouting" })
    }
}
