use crate::bridge::ChainedCommands;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(method, js_name = toggleParagraph)]
    pub fn toggle_paragraph(this: &ChainedCommands) -> ChainedCommands;
}
