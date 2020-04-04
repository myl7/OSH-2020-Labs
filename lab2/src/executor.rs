use crate::parser::{StdinType, StdoutMode, StdoutType};
use crate::{Cmd, Result};
use std::fs::{File, OpenOptions};
use std::process::{Child, Command, Stdio};

impl Cmd {
    pub fn execute(&self) -> Result<i32> {
        let mut piped = Option::<Child>::None;
        for sub_cmd in self.sub_cmds.iter() {
            let prog = sub_cmd.args.get(0).map(|s| s.as_str()).unwrap_or("");
            let args = &sub_cmd.args[1..];

            piped = Some(
                Command::new(prog)
                    .args(args)
                    .stdin(match &sub_cmd.stdin {
                        StdinType::Inherit => Stdio::inherit(),
                        StdinType::Piped => Stdio::from(piped.unwrap().stdout.unwrap()),
                        StdinType::Redirected(path) => Stdio::from(
                            File::open(path)
                                .map_err(|_| format!("Can not open file to read: {}", path))?,
                        ),
                    })
                    .stdout(match &sub_cmd.stdout {
                        StdoutType::Inherit => Stdio::inherit(),
                        StdoutType::Piped => Stdio::piped(),
                        StdoutType::Redirected(path, mode) => match mode {
                            StdoutMode::Overwrite => Stdio::from(
                                File::create(path)
                                    .map_err(|_| format!("Can not open file to write: {}", path))?,
                            ),
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
                    .map_err(|_| "Execution failed.")?,
            )
        }

        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}
