extern crate dirs;
extern crate regex;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
#[allow(unused_imports)]
#[macro_use]
extern crate quick_error;

pub mod error;
pub mod parser;

pub use error::{Error, Result};
pub use parser::Cmd;
