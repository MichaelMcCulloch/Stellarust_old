#![recursion_limit = "256"]

mod app;
mod fetch_service;
mod graph;
mod header;
mod ws_service;

pub use app::App;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    yew::start_app::<app::App>();
}
