use serde::Serialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use web_sys::{
    js_sys::{global, Array, Object, Reflect},
    Element,
};

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(js_namespace = Tiptap, js_name = Editor)]
    pub type Editor;

    #[wasm_bindgen(js_namespace = window, js_name = TiptapDocument)]
    fn TiptapDocument();

    #[wasm_bindgen(constructor, js_namespace = Tiptap)]
    fn _new(options: &JsValue) -> Editor;

    pub type ChainedCommands;

    #[wasm_bindgen(method)]
    pub fn chain(this: &Editor) -> ChainedCommands;

    #[wasm_bindgen(method)]
    pub fn focus(this: &ChainedCommands) -> ChainedCommands;

    #[wasm_bindgen(method)]
    pub fn run(this: &ChainedCommands) -> bool;
}

#[derive(Serialize)]
pub struct EditorOptions {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub element: Element,
    pub content: String,
    #[serde(skip)]
    pub extensions: Vec<Extension>,
}

#[derive(Serialize, strum_macros::Display)]
pub enum Extension {
    StarterKit,
    CustomExtension(#[serde(with = "serde_wasm_bindgen::preserve")] JsValue),
}

impl Into<JsValue> for Extension {
    fn into(self) -> JsValue {
        match self {
            Extension::CustomExtension(value) => value,
            _ => {
                let typename = self.to_string();
                Reflect::get(&global(), &JsValue::from_str(&typename)).unwrap()
            }
        }
    }
}

impl Editor {
    pub fn new(options: EditorOptions) -> Editor {
        let js_options = serde_wasm_bindgen::to_value(&options).unwrap();
        let js_options = js_options.unchecked_into::<Object>();

        let extensions = Array::new();

        for extension in options.extensions {
            extensions.push(&Into::<JsValue>::into(extension));
        }

        Reflect::set(&js_options, &"extensions".into(), &extensions).unwrap();

        Self::_new(&js_options)
    }
}
