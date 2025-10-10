mod bridge;
mod extensions;

pub mod prelude {
    pub use super::bridge::{Editor, EditorOptions, Extension::*};
}
