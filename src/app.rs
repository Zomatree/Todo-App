use dioxus::prelude::*;
use dioxus::router::{Router, Link, Route};

use crate::pages::{Todos, Account, Login, Register, Logout};
use crate::TOKEN;

pub fn App(cx: Scope) -> Element {
    let token_setter = use_set(&cx, TOKEN);

    cx.use_hook(|_| {
        let user_token = wasm_cookies::get("user_token").map(|res| res.unwrap());
        token_setter(user_token);
    });

    let user_token = use_read(&cx, TOKEN);

    rsx!(cx, Router {
        div {
            class: "navbar navbar-expand-lg navbar-light bg-light",
            div {
                class: "collapse navbar-collapse",
                ul {
                    class: "navbar-nav me-auto mb-2 mb-lg-0",
                    div {
                        class: "nav-item",
                        Link {
                            to: "/todos",
                            class: "nav-link",
                            "Todos"
                        },
                    }
                }
                if user_token.is_some() {
                    rsx!(
                        div {
                            style: "display: flex; flex-direction: row;",
                            div {
                                class: "nav-item",
                                Link {
                                    to: "/account",
                                    class: "nav-link",
                                    "Account"
                                },
                            }
                            div {
                                class: "nav-item",
                                Link {
                                    to: "/logout",
                                    class: "nav-link",
                                    "Logout"
                                },
                            }
                        }
                    )
                } else {
                    rsx!( Link {
                        to: "/login",
                        a {
                            class: "nav-link",
                            "Login"
                        }
                    })
                },
            }
        },
        Route {
            to: "/todos",
            Todos {

            },
        },
        Route {
            to: "/account",
            Account {

            }
        },
        Route {
            to: "/login",
            Login {

            }
        },
        Route {
            to: "/register",
            Register {

            }
        },
        Route {
            to: "/logout",
            Logout {

            }
        }
    })
}
