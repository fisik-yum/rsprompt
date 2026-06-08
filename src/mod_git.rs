use crate::modules;
use git2;

pub struct Git;
impl modules::Module for Git {
    fn fmt(opts: &Option<Vec<String>>) -> String {
        if let Some(o) = opts {
            if o.len() > 0 {
                let fmt_str = o.get(0).unwrap();
                let (b, c) = Git::stats();
                if b.is_none() || c.is_none() {
                    return "".to_string();
                }
                fmt_str
                    .replace("%b", b.unwrap().as_str())
                    .replace("%c", c.unwrap().as_str())
            } else {
                return String::from("");
            }
        } else {
            return String::from("");
        }
    }
}

impl Git {
    fn stats() -> (Option<String>, Option<String>) {
        let flags = git2::RepositoryOpenFlags::empty();
        let ceiling_dirs: Vec<String> = vec![]; // no ceiling limits

        let mut branch: Option<String> = None;
        let mut commit: Option<String> = None;
        match git2::Repository::open_ext(".", flags, ceiling_dirs) {
            Ok(repo) => {
                let head = repo.head().unwrap();
                if head.is_branch() {
                    if let Ok(b) = head.shorthand() {
                        branch = Some(b.to_string());
                    }
                }
                if let Ok(c) = &head.peel_to_commit() {
                    commit = Some(c.id().to_string()[..7].to_string());
                }
            }
            Err(_) => {}
        }

        return (branch, commit);
    }
}
