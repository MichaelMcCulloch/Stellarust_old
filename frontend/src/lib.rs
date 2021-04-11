mod app;
mod graph;
mod header;

pub use app::App;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<app::App>();
}
