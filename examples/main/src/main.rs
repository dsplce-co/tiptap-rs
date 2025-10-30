use gloo::utils::window;
use wasm_bindgen::prelude::*;
use web_sys::*;

mod start;
use start::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub(crate) fn console_log(s: &JsValue);
}

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let window = window();

    let callback = Closure::wrap(Box::new(start) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("DOMContentLoaded", callback.as_ref().unchecked_ref())
        .unwrap();

    callback.forget();
}
