#![allow(non_snake_case)]
#![feature(let_chains)]

use dioxus::{web::launch, fermi::Atom};

pub mod app;
pub mod pages;

pub static CLIENT: Atom<reqwest::Client> = |_| reqwest::Client::new();
pub static TOKEN: Atom<Option<String>> = |_| None;

pub const BASE_URL: &str = "http://localhost:8001";

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch(app::App)
}
