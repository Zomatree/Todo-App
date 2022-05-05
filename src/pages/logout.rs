use dioxus::prelude::*;
use wasm_cookies::{delete as delete_cookie};

use crate::TOKEN;

pub fn Logout(cx: Scope) -> Element {
    let router = use_router(&cx);
    let token_setter = use_set(&cx, TOKEN);

    delete_cookie("user_token");
    delete_cookie("user_name");
    token_setter(None);

    router.push_route("/", Some("Home".to_string()), None);
    rsx!(cx, div {
        "Logging out"
    })
}
