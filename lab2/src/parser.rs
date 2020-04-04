use crate::Result;
use std::io::{self, BufRead};

#[derive(Debug, Eq, PartialEq)]
pub struct Cmd {
    pub sub_cmds: Vec<SubCmd>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SubCmd {
    pub args: Vec<String>,
    pub stdin: StdinType,
    pub stdout: StdoutType,
}

#[derive(Debug, Eq, PartialEq)]
pub enum StdinType {
    Inherit,
    Piped,
    Redirected(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum StdoutType {
    Inherit,
    Piped,
    Redirected(String, StdoutMode),
}

#[derive(Debug, Eq, PartialEq)]
pub enum StdoutMode {
    Overwrite,
    Append,
}

impl Cmd {
    pub fn new() -> Result<Cmd> {
        Self::read(io::stdin().lock())
    }

    fn read(mut stdin: impl BufRead) -> Result<Cmd> {
        // Read one line.
        let mut cmd_str = String::new();
        stdin.read_line(&mut cmd_str)?;

        // Split by whitespaces and by "|".
        // To simplify parsing, force spaces between command elements.
        // i.e. `test | test` is ok but `test|test` not.
        let cmd_args = cmd_str.split_whitespace().collect::<Vec<&str>>();
        let sub_cmds_args = cmd_args.split(|&s| s == "|").collect::<Vec<&[&str]>>();

        // Build SubCmd vec.
        let sub_cmds_res =
            sub_cmds_args
                .into_iter()
                .map(|c| SubCmd::new(c))
                .fold(Ok(Vec::new()), |acc, r| match acc {
                    Err(e) => Err(e),
                    Ok(mut v) => match r {
                        Err(e) => Err(e),
                        Ok(c) => {
                            v.push(c);
                            Ok(v)
                        }
                    },
                });
        let mut sub_cmds = sub_cmds_res?;

        // Update piping.
        for i in 1..(sub_cmds.len() - 1) {
            sub_cmds[i].stdin = StdinType::Piped;
            sub_cmds[i].stdout = StdoutType::Piped;
        }
        if sub_cmds.get(0).is_some() {
            sub_cmds[0].stdout = StdoutType::Piped;
        }
        if sub_cmds.get(sub_cmds.len() - 1).is_some() {
            sub_cmds[0].stdin = StdinType::Piped;
        }

        Ok(Self { sub_cmds })
    }
}

impl SubCmd {
    fn new(args: &[&str]) -> Result<Self> {
        let mut skip = Vec::new();

        let stdin = match Self::get_stdio(&args, "<", &mut skip)? {
            None => StdinType::Inherit,
            Some(stdin) => StdinType::Redirected(stdin),
        };

        let stdout = match Self::get_stdio(&args, ">", &mut skip)? {
            Some(stdout) => StdoutType::Redirected(stdout, StdoutMode::Overwrite),
            None => match Self::get_stdio(&args, ">>", &mut skip)? {
                None => StdoutType::Inherit,
                Some(stdout) => StdoutType::Redirected(stdout, StdoutMode::Overwrite),
            },
        };

        Ok(Self {
            args: args
                .iter()
                .enumerate()
                .filter(|&(i, _)| skip.contains(&i))
                .map(|(_, &s)| s.to_string())
                .collect(),
            stdin,
            stdout,
        })
    }

    fn get_stdio(
        args: &[&str],
        sign: &'static str,
        skip: &mut Vec<usize>,
    ) -> Result<Option<String>> {
        match args
            .iter()
            .enumerate()
            .find(|&(_, &s)| s == sign)
            .map(|(i, _)| match args.get(i + 1) {
                None => Err(format!(
                    "Bad usage of {} at pos {}: Not followed with file path.",
                    sign, i
                )),
                Some(&s) => {
                    skip.append(&mut vec![i, i + 1]);
                    Ok(s.to_string())
                }
            }) {
            None => Ok(None),
            Some(res) => Ok(Some(res?)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}
