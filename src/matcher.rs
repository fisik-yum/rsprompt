use crate::{mod_git, mod_go, mod_rust, mod_text, modules::Module, token};
pub fn match_token<'a>(tok: &'a token::Token) -> String {
    match tok {
        token::Token::Text(s) => match *s {
            "user" => "\\u".to_string(),
            "host" => "\\h".to_string(),
            "date" => "\\d".to_string(),
            "device" => "\\l".to_string(),
            "shellname" => "\\s".to_string(),
            "time24" => "\\t".to_string(),
            "time12" => "\\T".to_string(),
            "time12m" => "\\@".to_string(),
            "cwd" => "\\w".to_string(),
            "basenum" => "\\W".to_string(),
            "cmdnum" => "\\#".to_string(),
            _ => s.to_string(),
        },
        token::Token::Module { name, opts } => match *name {
            "text" => mod_text::Text::fmt(opts),
            "git" => mod_git::Git::fmt(opts),
            "go" => mod_go::Go::fmt(opts),
            "rust" => mod_rust::Rust::fmt(opts),
            _ => name.to_string(),
        },
    }
}
