use crate::bridge::ChainedCommands;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(method, js_name = toggleBulletList)]
    pub fn toggle_bullet_list(this: &ChainedCommands) -> ChainedCommands;
}
