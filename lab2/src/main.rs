use msh::{exec, Cmd, Error};
use std::io::Write;
use std::{io, process};

fn main() -> ! {
    ctrlc::set_handler(move || {
        exec::clear_exec_pids();

        print!("\n# ");
        io::stdout().flush().expect("Fail to Flush stdout.");
    })
    .expect("Can not handle SIGINT which can be caused by Ctrl+C.");

    loop {
        print!("# ");
        io::stdout().flush().expect("Fail to Flush stdout.");

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
