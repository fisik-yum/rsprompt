use std::ffi::os_str;
use std::{ffi::OsStr, path::Path, process::Command};

use crate::modules;

pub struct Git;
impl modules::Module for Git {
    fn fmt(opts: &Option<Vec<String>>) -> String {
        if let Some(o) = opts {
            if o.len() > 0 {
                let fmt_str = o.get(0).unwrap();
                let (b, c) = Git::stats();
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
    fn stats() -> (String, String) {
        let flags = git2::RepositoryOpenFlags::empty();
        let ceiling_dirs: Vec<String> = vec![]; // no ceiling limits
        match git2::Repository::open_ext(".", flags, ceiling_dirs) {
            Ok(repo) => {
                let head = repo.head().unwrap();
                let branch: String;
                if head.is_branch() {
                    if let Ok(b) = head.shorthand() {
                        branch = b.to_string();
                    } else {
                        branch = "".to_string();
                    }
                } else {
                    branch = "".to_string();
                }
                let commit: String;
                if let Ok(c) = &head.peel_to_commit() {
                    commit = c.id().to_string()[..7].to_string();
                } else {
                    commit = "".to_string();
                }
                return (branch, commit.to_string());
            }
            Err(_) => return ("".to_string(), "".to_string()),
        }
    }
}
