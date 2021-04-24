#![recursion_limit = "256"]

mod app;
mod event_source_service;
mod fetch_service;
mod graph;
mod header;
mod ws_service;

pub use app::App;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}
