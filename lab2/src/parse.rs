use crate::Result;
use regex::Regex;
use std::io::{self, BufRead, Write};
use std::{env, mem};
use users;
use users::os::unix::UserExt;

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

lazy_static! {
    static ref TILDE_RE: Regex = Regex::new(r"^~(?P<user>[a-zA-Z0-9-_]*)").unwrap();
    static ref ENV_RE: Regex = Regex::new(r"\$(?P<var>[a-zA-Z0-9-_]+)").unwrap();
}

impl Cmd {
    pub fn new() -> Result<Cmd> {
        Self::parse(io::stdin().lock())
    }

    fn read_split(stdin: impl BufRead) -> Result<Vec<String>> {
        print!("# ");
        io::stdout().flush()?;

        let mut args = Vec::new();

        let mut in_arg = false;
        let mut arg = String::new();

        // Will keep in_arg = true when quote = Some(...) manually.
        let mut quote_sign = None;
        let find_quote = |c| ['\'', '"'].iter().find(|&&q| c == q);

        let mut is_end = true;

        let mut break_out = false;

        for line_res in stdin.lines() {
            let line = line_res? + "\n";

            for c in line.chars() {
                if c.is_whitespace() {
                    if let Some(_) = quote_sign {
                        arg.push(c);
                    } else if in_arg {
                        in_arg = false;
                        args.push(mem::replace(&mut arg, String::new()));
                    }
                } else {
                    if let Some(&q) = find_quote(c) {
                        if let Some(quote) = quote_sign {
                            if quote == q {
                                quote_sign = None;
                                is_end = true;
                            } else {
                                arg.push(c);
                            }
                        } else {
                            in_arg = true;
                            is_end = false;
                            quote_sign = Some(q);
                        }
                    } else {
                        in_arg = true;
                        arg.push(c);
                    }
                }
            }

            if is_end {
                break_out = true;
                break;
            } else {
                print!("> ");
                io::stdout().flush()?;
            }
        }

        if break_out {
            Ok(args)
        } else {
            Ok(vec!["exit".to_string()])
        }
    }

    fn parse(stdin: impl BufRead) -> Result<Cmd> {
        // To simplify parsing, force spaces between command elements.
        // i.e. `test > test` is ok but `test>test` not.
        let mut cmd_args = Vec::new();
        for s in Self::read_split(stdin)? {
            cmd_args.push(Self::insert_env(s)?);
        }

        let sub_cmds_args = cmd_args.split(|s| s == "|").collect::<Vec<&[String]>>();

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
        let len = sub_cmds.len();
        for i in 1..(len - 1) {
            sub_cmds[i].stdin = StdinType::Piped;
            sub_cmds[i].stdout = StdoutType::Piped;
        }
        if len > 1 {
            sub_cmds[0].stdout = StdoutType::Piped;
            sub_cmds[len - 1].stdin = StdinType::Piped;
        }

        Ok(Self { sub_cmds })
    }

    fn insert_env(mut s: String) -> Result<String> {
        if let Some(cap) = TILDE_RE.captures(&s) {
            let username = cap.name("user").unwrap().as_str();
            let range = cap.get(0).unwrap().range();

            let user = match username.as_ref() {
                "" => users::get_user_by_uid(users::get_current_uid()),
                username => users::get_user_by_name(&username),
            }
            .ok_or(format!("No such user: \"{}\".", &username))?;

            let home = user
                .home_dir()
                .to_str()
                .ok_or(format!(
                    "Can not parse got home dir of {} with UTF-8.",
                    &username
                ))?
                .to_string();
            drop(cap);

            s.replace_range(range, home.as_str());
        }

        let mut todo_envs = Vec::new();

        for cap in ENV_RE.captures_iter(&s) {
            let var = cap.name("var").unwrap().as_str().to_string();
            let range = cap.get(0).unwrap().range();
            todo_envs.push((var, range));
        }

        for (var, range) in todo_envs {
            let value = env::var(&var).unwrap_or("".to_string());
            s.replace_range(range, value.as_str());
        }

        Ok(s)
    }
}

impl SubCmd {
    fn new(args: &[String]) -> Result<Self> {
        let mut skip = Vec::new();

        let stdin = match Self::get_stdio(&args, "<", &mut skip)? {
            None => StdinType::Inherit,
            Some(stdin) => StdinType::Redirected(stdin),
        };

        let stdout = match Self::get_stdio(&args, ">", &mut skip)? {
            Some(stdout) => StdoutType::Redirected(stdout, StdoutMode::Overwrite),
            None => match Self::get_stdio(&args, ">>", &mut skip)? {
                None => StdoutType::Inherit,
                Some(stdout) => StdoutType::Redirected(stdout, StdoutMode::Append),
            },
        };

        Ok(Self {
            args: args
                .iter()
                .enumerate()
                .filter(|&(i, _)| !skip.contains(&i))
                .map(|(_, s)| s.clone())
                .collect(),
            stdin,
            stdout,
        })
    }

    fn get_stdio(
        args: &[String],
        sign: &'static str,
        skip: &mut Vec<usize>,
    ) -> Result<Option<String>> {
        match args
            .iter()
            .enumerate()
            .find(|&(_, s)| s == sign)
            .map(|(i, _)| match args.get(i + 1) {
                None => Err(format!(
                    "Bad usage of {} at pos {}: Not followed with file path.",
                    sign, i
                )),
                Some(s) => {
                    skip.append(&mut vec![i, i + 1]);
                    Ok(s.clone())
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
