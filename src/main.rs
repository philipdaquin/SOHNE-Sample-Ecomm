mod app;
mod routes;
mod components;
pub mod types;
pub mod api;
use crate::app::Main;

use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}