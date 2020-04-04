extern crate ctrlc;
extern crate dirs;
#[macro_use]
extern crate quick_error;

pub mod error;
pub mod exec;
pub mod parse;

pub use error::{Error, Result};
pub use parse::Cmd;
