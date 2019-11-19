#![recursion_limit = "512"]
#[macro_use]
extern crate stdweb;

mod app;
mod utils;
mod menu_block;
mod main_block;
mod myapp;

use wasm_bindgen::prelude::*;

use stdweb::web::{document, IElement, INode, IParentNode};


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    web_logger::init();
    yew::start_app::<myapp::MyApp>();
    // document().get_element_by_id(id: &str);
    // let mount_point = document().query_selector("#mount_point").unwrap().unwrap();
    // yew::initialize();
    // yew::App::<app::App>::new().mount(mount_point);
    // yew::run_loop();
    Ok(())
}
