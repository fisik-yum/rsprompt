use std::process::Command;

use crate::modules::Module;

pub struct Git;
impl Module for Git {
    fn fmt(opts: &Option<Vec<String>>) -> String {
        if let Some(o) = opts {
            if o.len() > 0 {
                let fmt_str = o.get(0).unwrap();
                let b = Git::branch();
                let c = Git::commit();
                if b == "" || c == "" {
                    return "".to_string();
                }
                fmt_str.replace("%b", b.as_str()).replace("%c", c.as_str())
            } else {
                return String::from("");
            }
        } else {
            return String::from("");
        }
    }
}

impl Git {
    fn branch() -> String {
        if let Ok(cmd) = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
        {
            if cmd.status.code().unwrap_or(-1) != 0 {
                return "".to_string();
            } else {
                let mut val = String::from_utf8(cmd.stdout).unwrap_or("".to_string());
                val = val
                    .strip_suffix("\r\n")
                    .or(val.strip_suffix("\n"))
                    .unwrap_or("")
                    .to_string();
                return val;
            }
        } else {
            return "".to_string();
        }
    }
    fn commit() -> String {
        if let Ok(cmd) = Command::new("git")
            .arg("rev-parse")
            .arg("--short")
            .arg("HEAD")
            .output()
        {
            if cmd.status.code().unwrap_or(-1) != 0 {
                return "".to_string();
            } else {
                let mut val = String::from_utf8(cmd.stdout).unwrap_or("".to_string());
                val = val
                    .strip_suffix("\r\n")
                    .or(val.strip_suffix("\n"))
                    .unwrap_or("")
                    .to_string();
                return val;
            }
        } else {
            return "".to_string();
        }
    }
}
