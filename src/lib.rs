/*!
# coilcalc

**coilcalc** is a small tool to estimate the magnetic field produced from a set of current
loops.

*/
#![recursion_limit = "512"]
pub mod current_loop;
pub mod utils;
use wasm_bindgen::prelude::*;

pub mod app;
use app::Model;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<Model>();
    Ok(())
}
