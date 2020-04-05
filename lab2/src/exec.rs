use crate::parse::{StdinType, StdoutMode, StdoutType};
use crate::{builtin, Cmd, Result};
use libc::{kill, pid_t, SIGINT};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};

thread_local! {
    static EXEC_PIDS: RefCell<Vec<u32>> = RefCell::new(Vec::new());
}

pub fn clear_exec_pids() {
    EXEC_PIDS.with(|c| {
        for pid in c.borrow().iter() {
            unsafe {
                kill(*pid as pid_t, SIGINT);
            }
        }

        c.borrow_mut().clear();
    });
}

impl Cmd {
    pub fn execute(&self) -> Result<i32> {
        let mut piped = (Option::<Child>::None, Option::<(i32, String)>::None);
        let mut is_builtin = false;

        for sub_cmd in self.sub_cmds.iter() {
            let prog = sub_cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            let args = if sub_cmd.args.len() > 1 {
                &sub_cmd.args[1..]
            } else {
                &[]
            };

            piped = if let Some(out) = match prog {
                "" => Some(builtin::noop()?),
                "cd" => Some(builtin::cd(args)?),
                "cwd" => Some(builtin::cwd()?),
                "export" => Some(builtin::export(args)?),
                "exit" => Some(builtin::exit(args)?),
                _ => None,
            } {
                is_builtin = true;
                (None, Some(out))
            } else {
                is_builtin = false;
                (
                    Some({
                        let mut child = Command::new(prog)
                            .args(args)
                            .stdin(match &sub_cmd.stdin {
                                StdinType::Inherit => Stdio::inherit(),
                                StdinType::Piped => {
                                    if is_builtin {
                                        Stdio::piped()
                                    } else {
                                        Stdio::from(piped.0.unwrap().stdout.unwrap())
                                    }
                                }
                                StdinType::Redirected(path) => {
                                    Stdio::from(File::open(path).map_err(|_| {
                                        format!("Can not open file to read: {}", path)
                                    })?)
                                }
                            })
                            .stdout(match &sub_cmd.stdout {
                                StdoutType::Inherit => Stdio::inherit(),
                                StdoutType::Piped => Stdio::piped(),
                                StdoutType::Redirected(path, mode) => match mode {
                                    StdoutMode::Overwrite => {
                                        Stdio::from(File::create(path).map_err(|_| {
                                            format!("Can not open file to write: {}", path)
                                        })?)
                                    }
                                    StdoutMode::Append => Stdio::from(
                                        OpenOptions::new()
                                            .append(true)
                                            .create(true)
                                            .open(path)
                                            .map_err(|_| {
                                                format!("Can not open file to append: {}", path)
                                            })?,
                                    ),
                                },
                            })
                            .spawn()
                            .map_err(|_| "Execution failed.")?;

                        // Save pid.
                        EXEC_PIDS.with(|c| {
                            c.borrow_mut().push(child.id());
                        });

                        // Pipe builtin command stdout.
                        if is_builtin {
                            child
                                .borrow_mut()
                                .stdin
                                .as_mut()
                                .unwrap()
                                .write_all(piped.1.unwrap().1.as_bytes())
                                .map_err(|e| e.to_string())?;
                        }

                        child
                    }),
                    None,
                )
            }
        }

        let exitcode = if is_builtin {
            print!("{}", piped.1.as_ref().unwrap().1);
            io::stdout().flush().map_err(|e| e.to_string())?;
            piped.1.as_ref().unwrap().0
        } else {
            piped
                .0
                .unwrap()
                .wait()
                .map_err(|e| e.to_string())?
                .code()
                .ok_or("Terminated by signal.")?
        };

        Ok(exitcode)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}
