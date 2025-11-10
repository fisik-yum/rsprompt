use std::process::Command;

use crate::modules::Module;

pub struct Go;

impl Module for Go {
    fn fmt(opts: &Option<Vec<String>>) -> String {
        if let Some(o) = opts {
            if o.len() > 0 {
                let fmt_str = o.get(0).unwrap();
                if let Some(m) = Go::get_module_name() {
                    fmt_str.replace("%s", m.as_str())
                } else {
                    "".to_string()
                }
            } else {
                return String::from("");
            }
        } else {
            return String::from("");
        }
    }
}

impl Go {
    fn get_module_name() -> Option<String> {
        if let Ok(cmd) = Command::new("go").arg("list").arg("-m").output() {
            let out = String::from_utf8(cmd.stdout).unwrap_or("".to_string());
            if !out.starts_with("command-line-arguments") {
                let val = out
                    .strip_suffix("\r\n")
                    .or(out.strip_suffix("\n"))
                    .unwrap_or("")
                    .to_string();
                return Some(val);
            } else {
                None
            }
        } else {
            None
        }
    }
}
