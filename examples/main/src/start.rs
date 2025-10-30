use gloo::utils::document;

use wasm_bindgen::{prelude::Closure, JsCast};

use web_sys::EventTarget;

use crate::alert;

macro_rules! hook_button {
    ($editor: ident, $name: ident) => {
        affix::paste! {
            let selector = concat!(".format--", stringify!($name));

            let element = gloo::utils::document().query_selector(selector).unwrap();

            let Some(element) = element else {
                alert(&format!("Element with class '{selector}' not found!"));
                return;
            };

            let editor = $editor.clone();

            let callback = Closure::wrap(Box::new(move |_: EventTarget| {
                editor.chain().focus().[<toggle_ $name>]().run();
            }) as Box<dyn FnMut(_)>);

            let _ = element.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref());

            callback.forget();
        }
    }
}

pub fn start(_: EventTarget) {
    pub use tiptap_rs::prelude::*;

    let document = document();

    let element = document.query_selector(".element").unwrap();

    let element = match element {
        Some(el) => el,
        None => {
            alert("Element with class 'element' not found!");
            return;
        }
    };

    tracing::info!("Editor initialized");

    let options = EditorOptions {
        element,
        content: "<p>Hello from Rust!</p>".to_string(),
        extensions: vec![StarterKit],
    };

    let editor = Editor::new(options);

    hook_button!(editor, bold);
    hook_button!(editor, italic);
    hook_button!(editor, strike);
    hook_button!(editor, bullet_list);
    hook_button!(editor, ordered_list);

    hook_button!(editor, h1);
    hook_button!(editor, h2);
    hook_button!(editor, h3);
    hook_button!(editor, h4);
    hook_button!(editor, h5);
    hook_button!(editor, h6);
}
