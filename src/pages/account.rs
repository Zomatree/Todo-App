use dioxus::prelude::*;

pub fn Account(cx: Scope) -> Element {
    rsx!(cx, div {
        h1 {
            "accounts"
        }
    })
}
