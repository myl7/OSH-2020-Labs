extern crate ctrlc;
extern crate dirs;
extern crate regex;
extern crate users;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate lazy_static;

pub mod builtin;
pub mod error;
pub mod exec;
pub mod parse;

pub use error::{Error, Result};
pub use parse::Cmd;
