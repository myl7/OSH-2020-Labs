use msh::{exec, Cmd, Error};

fn main() -> ! {
    ctrlc::set_handler(move || {
        exec::clear_exec_pids();
        println!();
    })
    .expect("Can not handle SIGINT which can be caused by Ctrl+C.");

    loop {
        match Cmd::new() {
            Err(e) => match e {
                Error::ReadFailed => panic!("Failed to read."),
                Error::BadCmd(reason) => eprintln!("{}", reason),
            },
            Ok(cmd) => match cmd.execute() {
                Err(e) => match e {
                    Error::ReadFailed => panic!("Failed to read."),
                    Error::BadCmd(reason) => eprintln!("{}", reason),
                },
                Ok(code) => eprintln!("Exited with {}.", code),
            },
        }
    }
}
