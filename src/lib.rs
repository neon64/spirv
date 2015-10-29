extern crate libc;

mod instruction;
mod module;
pub mod enumerations;

pub use instruction::data::*;
pub use module::{Module, ModuleHeader};
pub use instruction::*;