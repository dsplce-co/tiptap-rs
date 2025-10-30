> We're dsplce.co, check out our work on [github.com/dsplce-co](https://github.com/dsplce-co) 🖤

# tiptap-rs

✍️ Type-safe Wasm bindings for the [Tiptap](https://tiptap.dev/) headless rich text editor. This crate's API is designed to reflect Tiptap's original JavaScript API as faithfully as possible, while also providing a more idiomatic Rust experience.

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
tiptap-rs = "0.1.3"
```

## Setup

Add the following script to your HTML `<head>` tag to make Tiptap available to your Wasm module:

```html
<script type="module">
    import * as Tiptap from "https://esm.sh/@tiptap/core";
    import StarterKit from "https://esm.sh/@tiptap/starter-kit";

    window.Tiptap = Tiptap;
    window.StarterKit = StarterKit;
</script>
```

---

## 🧪 Usage

### 1. Basic Editor Setup

Create a Tiptap editor:

```rust
use tiptap_rs::prelude::*;
use web_sys::*;

let document = gloo::utils::document();
let element = document.query_selector(".editor").unwrap().unwrap();

let options = EditorOptions {
    element,
    content: "<p>Hello from Rust!</p>".to_string(),
    extensions: vec![StarterKit],
};

let editor = Editor::new(options);
```

### 2. Using Chained Commands

Tiptap's original API is faithfully mirrored to provide a seamless transition from JS:

```rust
// Toggle bold formatting
editor.chain().focus().toggle_bold().run();

// Toggle italic formatting
editor.chain().focus().toggle_italic().run();
```

Some methods deviate from their original counterparts for ease of use, e.g.:

```rust
// Toggle heading level
// Note: One method per level, original is `.toggleHeading({ level: 1 })`
editor.chain().focus().toggle_h1().run();
```

### 3. Handling Button Events

Connect editor commands to UI buttons:

```rust
use wasm_bindgen::prelude::*;

let editor_clone = editor.clone();
let callback = Closure::wrap(Box::new(move |_| {
    editor_clone.chain().focus().toggle_bold().run();
}) as Box<dyn FnMut(_)>);

let bold_button = document.query_selector(".bold-button").unwrap().unwrap();
bold_button.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()).unwrap();
callback.forget();
```

### 4. Supported Extensions

The following Tiptap extensions have Rust bindings:

- **Text Formatting**: Bold, Italic, Strike-through
- **Lists**: Bullet lists, Ordered lists
- **Headings**: H1 through H6
- **Basic Blocks**: Paragraphs
- **StarterKit**: Complete bundle of common extensions

---

## 📐 API Reference

### `Editor`

The main editor instance that wraps Tiptap's Editor class.

#### `Editor::new(options: EditorOptions) -> Editor`

Creates a new editor instance with the specified options.

### `EditorOptions`

Configuration structure for instantiating an editor:

```rust
pub struct EditorOptions {
    pub element: Element,
    pub content: String,
    pub extensions: Vec<Extension>,
}
```

### Chained Commands

Currently supported `ChainedCommands`:

- `toggle_bold()`
- `toggle_italic()`
- `toggle_strike()`
- `toggle_h1()` through `toggle_h6()`
- `toggle_bullet_list()`
- `toggle_ordered_list()`
- `focus()`
- `run()` - Execute the command chain

---

## 🚀 Examples

Run the examples:

```bash
cargo make serve
```

This requires [cargo-make](https://crates.io/crates/cargo-make) to be installed.

The example demonstrates:
- Basic editor setup
- Formatting commands usage on UI events
- Extension usage with StarterKit

---

## 📁 Repo & Contributions

📦 Crate: [crates.io/crates/tiptap-rs](https://crates.io/crates/tiptap-rs)<br>
🛠️ Repo: [github.com/dsplce-co/tiptap-rs](https://github.com/dsplce-co/tiptap-rs)

PRs welcome! 🖤

---

## 🔒 License

MIT or Apache-2.0, at your option.

---
