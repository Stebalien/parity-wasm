//! Various builders to generate/alter wasm components

mod invoke;
mod module;
mod code;
mod misc;

pub use self::module::{module, ModuleBuilder};
pub use self::code::signatures;