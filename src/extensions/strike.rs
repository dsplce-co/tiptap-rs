use crate::bridge::ChainedCommands;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(method, js_name = toggleStrike)]
    pub fn toggle_strike(this: &ChainedCommands) -> ChainedCommands;
}
