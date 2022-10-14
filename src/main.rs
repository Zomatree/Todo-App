#![allow(non_snake_case)]
#![feature(let_chains, async_closure)]

use dioxus::{web::launch, fermi::Atom};
use dioxus::prelude::*;
use futures::{Future, channel::mpsc::{self, UnboundedReceiver, UnboundedSender}};
pub mod app;
pub mod pages;

pub static CLIENT: Atom<reqwest::Client> = |_| reqwest::Client::new();
pub static TOKEN: Atom<Option<String>> = |_| None;

pub const BASE_URL: &str = "http://localhost:8001";

pub fn use_both_coroutine<M, G, F>(cx: &ScopeState, init: G) -> &CoroutineHandle<M>
where
    M: 'static,
    G: FnOnce(UnboundedReceiver<M>, CoroutineHandle<M>) -> F,
    F: Future<Output = ()> + 'static,
{
    cx.use_hook(|_| {
        let (tx, rx) = mpsc::unbounded();
        let handle = CoroutineHandle(tx);

        cx.push_future(init(rx, handle.clone()));
        cx.provide_context(handle)
    })
}

pub struct CoroutineHandle<T>(UnboundedSender<T>);

impl<T> Clone for CoroutineHandle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> CoroutineHandle<T> {
    pub fn send(&self, msg: T) {
        let _ = self.0.unbounded_send(msg);
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch(app::App)
}
