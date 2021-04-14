#![recursion_limit = "256"]


mod app;
mod graph;
mod header;
mod fetch_service;

pub use app::App;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<app::App>();
}
