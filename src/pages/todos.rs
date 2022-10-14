use dioxus::prelude::*;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use eos::{DateTime, Utc, format_dt};
use futures::StreamExt;

use crate::{CLIENT, BASE_URL, TOKEN, use_both_coroutine};

#[derive(Debug, Deserialize, Clone)]
struct Todo {
    pub title: String,
    pub id: String,
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


#[derive(Debug, Serialize)]
struct EditTodo {
    #[serde(skip_serializing_if="Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    complete: Option<bool>
}

enum TodoAction {
    GetAll,
    Edit {
        todo_id: String,
        edit: EditTodo
    },
    Delete(String),
    Create(String),
}

#[derive(Clone)]
enum ModalState {
    None,
    CreateTodo(String),
    EditTodo {
        todo_id: String,
        title: String,
        description: String
    }
}

// i can clone the token around because it wont change between renders, the user needs to login which involves going to a diff
// route which means the hook here will rerun with the token not causing invalid state

pub fn Todos(cx: Scope) -> Element {
    let client = use_read(&cx, CLIENT);
    let user_token = use_read(&cx, TOKEN);

    let router = use_router(&cx);
    let todos_state = use_state(&cx, Vec::<Todo>::new);
    let modal_state = use_state(&cx, || ModalState::None);

    let coro_handler = use_both_coroutine::<TodoAction, _, _>(&cx, |mut rx, handle| {
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
                    TodoAction::Edit { todo_id, edit } => {
                        client.patch(format!("{BASE_URL}/api/todos/{todo_id}"))
                            .header("Authorization", token)
                            .json(&edit)
                            .send()
                            .await
                            .unwrap();

                        handle.send(TodoAction::GetAll);
                    },
                    TodoAction::Delete(todo_id) => {
                        client.delete(format!("{BASE_URL}/api/todos/{todo_id}"))
                            .header("Authorization", token)
                            .send()
                            .await
                            .unwrap();

                        handle.send(TodoAction::GetAll);
                    },
                    TodoAction::Create(title) => {
                        client
                            .post(format!("{BASE_URL}/api/todos"))
                            .header("Authorization", token)
                            .json(&CreateTodosRequest { title })
                            .send()
                            .await
                            .unwrap();

                        handle.send(TodoAction::GetAll);
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
                    onclick: move |_| {
                        log::info!("asdasdasd");
                        modal_state.set(ModalState::CreateTodo("".to_string()))
                    },
                    class: "d-flex btn btn-primary",
                    "+ New",
                }
            },
            match modal_state.get() {
                ModalState::None => rsx!{ None::<()> },
                _ => rsx! {
                    div {
                        tabindex: "2",
                        div {
                            class: "modal-dialog",
                            style: "pointer-events: all!important",
                            div {
                                class: "modal-contents",
                                div {
                                    class: "modal-header",
                                    h1 {
                                        match modal_state.get() {
                                            ModalState::None => rsx!(""),
                                            ModalState::CreateTodo { .. } => rsx!("Create Todo"),
                                            ModalState::EditTodo { .. } => rsx!("Edit Todo"),
                                        }
                                    },
                                    button {
                                        class: "btn-close",
                                    }
                                },
                                div {
                                    class: "modal-body",
                                    match modal_state.get() {
                                        ModalState::None => rsx!(""),
                                        ModalState::CreateTodo(_) => {
                                            rsx! {
                                                label {
                                                    "Title"
                                                },
                                                input {
                                                    r#type: "text",
                                                    oninput: move |evt| {
                                                        modal_state.set(ModalState::CreateTodo(evt.value.clone()))
                                                    }
                                                }
                                            }
                                        },
                                        ModalState::EditTodo { todo_id, title, description, .. } => {
                                            rsx! {
                                                label {
                                                    "Title"
                                                },
                                                input {
                                                    r#type: "text",
                                                    value: "{title}",
                                                    oninput: move |evt| {
                                                        modal_state.set(ModalState::EditTodo { todo_id: todo_id.clone(), title: evt.value.clone(), description: description.clone() })
                                                    }
                                                },
                                                label {
                                                    "Description"
                                                },
                                                input {
                                                    r#type: "text",
                                                    value: "{description}",
                                                    oninput: move |evt| {
                                                        modal_state.set(ModalState::EditTodo { todo_id: todo_id.clone(), title: title.clone(), description: evt.value.clone() })
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                div {
                                    class: "modal-footer",
                                    button {
                                        onclick: move |_| modal_state.set(ModalState::None),
                                        class: "btn btn-secondary",
                                        "Close"
                                    },
                                    button {
                                        onclick: move |_| {
                                            match modal_state.get().clone() {
                                                ModalState::None => unreachable!(),
                                                ModalState::CreateTodo(title) => {
                                                    coro_handler.send(TodoAction::Create(title));
                                                },
                                                ModalState::EditTodo { todo_id, title, description } => {
                                                    coro_handler.send(TodoAction::Edit {
                                                        todo_id,
                                                        edit: EditTodo {
                                                            title: Some(title),
                                                            description: Some(description),
                                                            complete: None
                                                        }
                                                    });
                                                }
                                            };
                                            modal_state.set(ModalState::None);
                                        },
                                        class: "btn btn-primary",
                                        "Done"
                                    }
                                }
                            }
                        }
                    }
                }
            },
            ol {
                class: "list-group list-group-numbered",
                todos_state.get().iter().map(|todo| {
                    let Todo { title, id, completed, created_at, description } = todo;
                    let formatted_date = format_dt!("%H:%M - %d/%m/%Y", created_at);

                    rsx! {
                        div {
                            class: "list-group-item",
                            key: "{id}",

                            div {
                                class: "d-flex flex-row justify-content-start align-items-center",
                                input {
                                    class: "form-check-input p-2",
                                    r#type: "checkbox",
                                    checked: "{completed}",
                                    oninput: move |_| {
                                        coro_handler.send(TodoAction::Edit {
                                            todo_id: id.clone(),
                                            edit: EditTodo {
                                                title: None,
                                                description: None,
                                                complete: Some(!completed)
                                            }
                                        });
                                    }
                                },
                                div {
                                    class: "d-flex flex-column p-2 me-auto",
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
                                    class: "d-flex btn btn-primary p-2",
                                    onclick: move |_| {
                                        modal_state.set(ModalState::EditTodo { todo_id: id.clone(), title: title.clone(), description: description.as_ref().cloned().unwrap_or_default() })
                                    },
                                    span { "Edit" },
                                }
                                button {
                                    class: "d-flex btn btn-danger p-2",
                                    onclick: |_| {
                                        coro_handler.send(TodoAction::Delete(id.clone()))
                                    },
                                    span { "delete" },
                                }
                            },
                        }
                    }
                })
            },
        })
    } else {
        router.push_route("/login", None, None);
        rsx!(cx, div { "Rerouting" })
    }
}
