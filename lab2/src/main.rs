use msh::{Cmd, Error};
use std::process;

fn main() -> ! {
    loop {
        match Cmd::new() {
            Err(e) => match e {
                Error::ReadFailed => process::exit(-1),
                Error::BadCmd(reason) => eprintln!("{}", reason),
            },
            Ok(cmd) => match cmd.execute() {
                Err(e) => match e {
                    Error::ReadFailed => process::exit(-1),
                    Error::BadCmd(reason) => eprintln!("{}", reason),
                },
                Ok(code) => eprintln!("Exited with {}.", code),
            },
        }
    }
}
