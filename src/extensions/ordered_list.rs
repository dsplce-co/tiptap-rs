use crate::bridge::ChainedCommands;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(method, js_name = toggleOrderedList)]
    pub fn toggle_ordered_list(this: &ChainedCommands) -> ChainedCommands;
}
