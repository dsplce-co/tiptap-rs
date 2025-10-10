use crate::bridge::ChainedCommands;
use serde::Serialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::js_sys::Object;

#[derive(Serialize)]
struct HeadingConfig {
    level: u8,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(method, js_name = toggleHeading)]
    fn _toggle_heading(this: &ChainedCommands, config: &JsValue) -> ChainedCommands;
}

macro_rules! heading_method {
    ($level: literal) => {
        affix::paste! {
            pub fn [<toggle_h $level>](self: ChainedCommands) -> ChainedCommands {
                let config = HeadingConfig { level: $level };

                let config = serde_wasm_bindgen::to_value(&config)
                    .unwrap()
                    .unchecked_into::<Object>();

                self._toggle_heading(&config)
            }
        }
    };
}

macro_rules! heading_methods {
    () => {
        heading_method!(1);
        heading_method!(2);
        heading_method!(3);
        heading_method!(4);
        heading_method!(5);
        heading_method!(6);
    };
}

impl ChainedCommands {
    heading_methods!();
}
