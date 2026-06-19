> We're dsplce.co, check out our work on our website: [dsplce.co](https://dsplce.co) 🖤

# tiptap-rs

[![Tiptap](https://img.shields.io/badge/Tiptap-6A00F5?style=for-the-badge&logo=tiptap&logoColor=white)](https://tiptap.dev/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=webassembly&logoColor=white)](https://webassembly.org/)
[![crates.io Downloads](https://img.shields.io/crates/d/tiptap-rs?style=for-the-badge&color=%23FF0346)](https://crates.io/crates/tiptap-rs)
[![crates.io Size](https://img.shields.io/crates/size/tiptap-rs?style=for-the-badge)](https://crates.io/crates/tiptap-rs)
[![License](https://img.shields.io/crates/l/tiptap-rs.svg?style=for-the-badge)](https://crates.io/crates/tiptap-rs)
[![crates.io](https://img.shields.io/crates/v/tiptap-rs?style=for-the-badge&color=%230F80C1)](https://crates.io/crates/tiptap-rs)

✍️ Type-safe Wasm bindings for the [Tiptap](https://tiptap.dev/) headless rich text editor.

`tiptap-rs` mirrors Tiptap's original JavaScript API as faithfully as possible, so a transition from JS feels like a transliteration rather than a rewrite — whilst handing you an idiomatic, type-safe Rust experience on top.

_Disclaimer: this project has no affiliation with the official Tiptap project or trademark._

## 🖤 Features

- **`editor.chain().focus().toggle_bold().run()`** — the chained-command API is mirrored 1:1 from Tiptap's JS, so muscle memory carries over and there's nothing new to learn
- **Type-safe `EditorOptions`** — configure the editor with a plain Rust struct instead of poking at an untyped JS object and hoping for the best
- **`StarterKit` in, batteries included** — the same common-extensions bundle you'd reach for in JS, one enum variant away
- **`CustomExtension(...)` — bring your own** — anything Tiptap can load, you can hand straight through; you're not boxed into what we happened to wrap
- **Headings as one method per level** — `toggle_h1()` through `toggle_h6()`, because `.toggleHeading({ level })` doesn't translate cleanly and we'd rather it read like Rust

---

## Table of Contents

- [🖤 Features](#-features)
- [📦 Installation](#-installation)
  - [cargo](#cargo)
  - [Setup](#setup)
- [🧪 Usage](#-usage)
  - [Create an editor](#create-an-editor)
  - [Chained commands](#chained-commands)
  - [Toggling headings](#toggling-headings)
  - [Custom extensions](#custom-extensions)
  - [Handling button events](#handling-button-events)
- [📐 API Reference](#-api-reference)
- [🚀 Examples](#-examples)
- [🛠️ Requirements](#%EF%B8%8F-requirements)
- [📁 Repo & Contributions](#-repo--contributions)
- [📄 License](#-license)

⸻

## 📦 Installation

### cargo

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
tiptap-rs = "0.1"
```

Or let cargo do it for you:

```bash
cargo add tiptap-rs
```

### Setup

`tiptap-rs` binds to Tiptap running in the browser, so Tiptap itself has to be on the page. Add the following to your HTML `<head>` to make it available to your Wasm module:

```html
<script type="module">
    import * as Tiptap from "https://esm.sh/@tiptap/core";
    import StarterKit from "https://esm.sh/@tiptap/starter-kit";

    window.Tiptap = Tiptap;
    window.StarterKit = StarterKit;
</script>
```

The bindings resolve `Tiptap` and any extension (like `StarterKit`) off the global object by name — so whatever you want to use from Rust, expose it on `window` here first.

⸻

## 🧪 Usage

### Create an editor

```rust
use tiptap_rs::prelude::*;
use gloo::utils::document;

let element = document
    .query_selector(".editor")
    .unwrap()
    .unwrap();

let options = EditorOptions {
    element,
    content: "<p>Hello from Rust!</p>".to_string(),
    extensions: vec![StarterKit],
};

let editor = Editor::new(options);
```

### Chained commands

Tiptap's original API is faithfully mirrored, so the chained commands read just like they do in JS:

```rust
// Toggle bold formatting
editor.chain().focus().toggle_bold().run();

// Toggle italic formatting
editor.chain().focus().toggle_italic().run();

// Toggle a paragraph block
editor.chain().focus().toggle_paragraph().run();
```

### Toggling headings

Headings are the one place we deviate from the JS API on purpose: instead of `.toggleHeading({ level: 1 })` you get one method per level, so it stays type-safe and reads like Rust:

```rust
editor.chain().focus().toggle_h1().run();
editor.chain().focus().toggle_h2().run();
// ... through toggle_h6()
```

### Custom extensions

`StarterKit` covers the common cases, but you're not limited to it. Expose any Tiptap extension on the window in your setup script:

```html
<script type="module">
    import Highlight from "https://esm.sh/@tiptap/extension-highlight";
    window.Highlight = Highlight;
</script>
```

then hand it through with `CustomExtension`, which resolves it off the global object by name:

```rust
use tiptap_rs::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::js_sys::{global, Reflect};

let highlight = Reflect::get(&global(), &JsValue::from_str("Highlight")).unwrap();

let options = EditorOptions {
    element,
    content: "<p>Hello from Rust!</p>".to_string(),
    extensions: vec![StarterKit, CustomExtension(highlight)],
};

let editor = Editor::new(options);
```

### Handling button events

Connect editor commands to UI buttons:

```rust
use wasm_bindgen::prelude::*;

let editor_clone = editor.clone();
let callback = Closure::wrap(Box::new(move |_| {
    editor_clone.chain().focus().toggle_bold().run();
}) as Box<dyn FnMut(_)>);

let bold_button = document.query_selector(".bold-button").unwrap().unwrap();
bold_button
    .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
    .unwrap();
callback.forget();
```

⸻

## 📐 API Reference

### `Editor`

The main editor instance, wrapping Tiptap's `Editor` class.

```rust
Editor::new(options: EditorOptions) -> Editor
```

Creates a new editor instance with the specified options. `Editor` is `Clone`, so you can hand copies into event callbacks.

### `EditorOptions`

Configuration struct for instantiating an editor:

```rust
pub struct EditorOptions {
    pub element: Element,
    pub content: String,
    pub extensions: Vec<Extension>,
}
```

### `Extension`

The extension set passed to an editor:

```rust
pub enum Extension {
    StarterKit,
    CustomExtension(JsValue),
}
```

- `StarterKit` — Tiptap's bundle of common extensions, resolved off the global object by name.
- `CustomExtension(JsValue)` — any other Tiptap extension you've exposed on `window` (see [Custom extensions](#custom-extensions)).

Both variants are re-exported from the prelude, so `StarterKit` and `CustomExtension(...)` are in scope directly.

### Chained commands

Currently supported on `ChainedCommands`:

- `toggle_bold()`
- `toggle_italic()`
- `toggle_strike()`
- `toggle_paragraph()`
- `toggle_bullet_list()`
- `toggle_ordered_list()`
- `toggle_h1()` through `toggle_h6()`
- `focus()`
- `run()` — execute the command chain (returns `bool`)

Start a chain with `editor.chain()`.

⸻

## 🚀 Examples

Run the bundled example:

```bash
cargo make serve
```

This requires [cargo-make](https://crates.io/crates/cargo-make); it pulls in [trunk](https://trunkrs.dev/) on first run to build and serve the Wasm.

The example demonstrates:

- Basic editor setup
- Formatting commands wired to UI buttons
- Extension usage with `StarterKit`

⸻

## 🛠️ Requirements

- **A `wasm32-unknown-unknown` target** — `tiptap-rs` only makes sense compiled to Wasm and run in a browser. Add it with `rustup target add wasm32-unknown-unknown`.
- **Tiptap on the page** — the matching `@tiptap/*` modules exposed on `window` (see [Setup](#setup)); the bindings call into them at runtime.
- **A bundler that loads it all** — anything that ships your Wasm alongside the HTML works; the example uses [trunk](https://trunkrs.dev/).

⸻

## 📁 Repo & Contributions

📦 **Crate**: [https://crates.io/crates/tiptap-rs](https://crates.io/crates/tiptap-rs)<br>
🛠️ **Repo**: [https://github.com/dsplce-co/tiptap-rs](https://github.com/dsplce-co/tiptap-rs)

PRs welcome 🖤

⸻

## 📄 License

MIT or Apache-2.0, at your option.
