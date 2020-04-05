use crate::Result;
use regex::Regex;
use std::{env, process};

pub fn noop() -> Result<(i32, String)> {
    Ok((0, "".to_string()))
}

fn cwd_inner() -> Result<String> {
    Ok(dirs::home_dir()
        .ok_or("Get current dir failed.")?
        .to_str()
        .ok_or("Get current dir failed: For UTF-8 validity.")?
        .to_string())
}

pub fn cd(args: &[String]) -> Result<(i32, String)> {
    // env::home_dir is deprecated since 1.29.0 for UB. Use crate `dirs`.
    let dir = args.get(0).map(|s| s.to_string()).unwrap_or(cwd_inner()?);
    env::set_current_dir(dir).map_err(|e| e.to_string())?;
    Ok((0, "".to_string()))
}

pub fn cwd() -> Result<(i32, String)> {
    Ok((0, cwd_inner()? + "\n"))
}

lazy_static! {
    static ref EXPORT_ARG_RE: Regex =
        Regex::new(r"^(?P<key>[_a-zA-Z][_a-zA-Z0-9]*)=(?P<value>.*)$").unwrap();
}

pub fn export(args: &[String]) -> Result<(i32, String)> {
    if args.len() > 0 {
        let mut cap_list = Vec::new();
        for arg in args {
            let cap = EXPORT_ARG_RE
                .captures(arg)
                .ok_or("Invalid assignment: ".to_string() + &arg)?;
            cap_list.push(cap);
        }

        for cap in cap_list {
            env::set_var(
                cap.name("key").unwrap().as_str(),
                cap.name("value").unwrap().as_str(),
            );
        }

        Ok((0, "".to_string()))
    } else {
        Ok((
            0,
            env::vars()
                .map(|(k, v)| format!("{}={}\n", k, v))
                .fold(String::new(), |acc, s| acc + s.as_str())
                + "\n",
        ))
    }
}

pub fn exit(args: &[String]) -> Result<(i32, String)> {
    let exit_code = if args.len() <= 1 {
        args.first()
            .map(|a| a.parse::<i32>().map_err(|e| e.to_string()))
            .unwrap_or(Ok(0))?
    } else {
        Err(format!("Too many arguments: {} arguments.", args.len()))?
    };

    process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
}
