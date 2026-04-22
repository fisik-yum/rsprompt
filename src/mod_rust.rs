use crate::modules;
use cargo_metadata::MetadataCommand;

pub struct Rust;

impl modules::Module for Rust {
    fn fmt(opts: &Option<Vec<String>>) -> String {
        if let Some(o) = opts {
            if o.len() > 0 {
                let fmt_str = o.get(0).unwrap();
                if let Some(m) = Rust::get_module_name() {
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

impl Rust {
    fn get_module_name() -> Option<String> {
        if let Ok(meta) = MetadataCommand::new().exec() {
            if let Some(pack) = meta.root_package() {
                Some(pack.name.to_string())
            } else {
                None
            }
        } else {
            None
        }
    }
}
