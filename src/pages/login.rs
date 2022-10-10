use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use futures::{StreamExt};
use wasm_cookies::{set as set_cookie, CookieOptions};
use reqwest::StatusCode;

use crate::{CLIENT, TOKEN, BASE_URL};

#[derive(Serialize)]
pub struct LoginBody {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct LoginResponse {
    name: String,

    #[allow(dead_code)]
    user_id: u64,

    token: String
}

pub fn Login(cx: Scope) -> Element {
    let client = use_read(&cx, CLIENT);
    let error_state = use_state(&cx, || None::<String>);
    let router = use_router(&cx);
    let write_token = use_set(&cx, TOKEN);

    let channel = use_coroutine::<(String, String), _, _>(&cx, move |mut rx| {
        let error_state = error_state.clone();
        let client = client.clone();
        let router = router.clone();
        let write_token = write_token.clone();

        async move {
            while let Some((username, password)) = rx.next().await {
                let resp = client.post(format!("{BASE_URL}/api/accounts/login")).json(&LoginBody {
                    username,
                    password,
                }).send().await;

                match resp {
                    Ok(response) => {
                        if response.status() != StatusCode::OK {
                            error_state.set(Some("Incorrect login details.".to_string()));
                        } else {
                            match response.json().await {
                                Ok(data) => {
                                    let LoginResponse { name, user_id: _, token } = data;
                                    let cookie_options = CookieOptions::default();

                                    set_cookie("user_name", &name, &cookie_options);
                                    set_cookie("user_token", &token, &cookie_options);

                                    write_token(Some(token));

                                    router.push_route("/account", Some("Account".to_string()), None);
                                },
                                Err(error) => {
                                    error_state.set(Some(error.to_string()));
                                }
                            }
                        };
                    },
                    Err(error) => {
                        error_state.set(Some(error.to_string()))
                    }
                }
            }
        }
    });

    rsx!(cx, section {
        class: "position-relative py-4 py-xl-5",
        div {
            class: "container",
            div {
                class: "row mb-5",
                div {
                    class: "col-md-8 col-xl-6 text-center mx-auto",
                    h2 {
                        "Log in"
                    }
                }
            }
            div {
                class: "row d-flex justify-content-center",
                div {
                    class: "col-md-6 col-xl-4",
                    div {
                        class: "card mb-5",
                        div {
                            class: "card-body d-flex flex-column align-items-center",
                            form {
                                class: "text-center",
                                prevent_default: "onsubmit",

                                onsubmit: move |evt| {
                                    let username = evt.values["username"].clone();
                                    let password = evt.values["password"].clone();

                                    channel.send((username, password));
                                },
                                div {
                                    class: "mb-3",
                                    input {
                                        class: "form-control",
                                        name: "username",
                                        r#type: "username",
                                        placeholder: "username",
                                    }
                                }
                                div {
                                    class: "mb-3",
                                    input {
                                        class: "form-control",
                                        name: "password",
                                        placeholder: "Password",
                                        r#type: "password",
                                    }
                                }
                                div {
                                    class: "mb-3",
                                    button {
                                        class: "btn btn-primary d-block w-100",
                                        r#type: "submit",
                                        "Login"
                                    }
                                }
                                p {
                                    class: "text-danger", error_state.get().clone() },
                                p {
                                    class: "text-muted",
                                    "Or "
                                    a {
                                        href: "/register",
                                        "Register"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}
