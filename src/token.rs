use std::fmt;
use std::fmt::Display;

use crate::matcher::match_token;
pub enum Token {
    Text(String),
    Module {
        name: String,
        opts: Option<Vec<String>>,
    },
}

impl Display for Token {
    // add code here
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Text(s) => write!(f, "Text: {s}"),
            Token::Module { name, opts } => match opts {
                Some(inner) => write!(f, "Module: {name} Opts: {}", inner.join(" ")),
                None => write!(f, "Module: {name} (no opts)",),
            },
        }
    }
}

pub fn parse(dat: &String) -> String {
    let tokens = tokenize(dat);
    let mapped_tokens: Vec<String> = tokens.iter().map(|tok| match_token(tok)).collect();
    mapped_tokens.join("")
}
// TODO: panics
fn tokenize(dat: &String) -> Vec<Token> {
    let mut ret: Vec<Token> = vec![];
    let as_chars = dat.chars();

    let mut res = String::from("");
    let mut is_open = false;
    for ch in as_chars {
        if ch == '{' {
            if is_open {
                panic!("malformed braces!")
            }
            if !res.is_empty() {
                ret.push(Token::Text(res.clone()));
                res.clear();
            }
            is_open = !is_open;
        } else if ch == '}' {
            if !is_open {
                panic!("malformed braces!")
            }
            if !res.is_empty() {
                ret.push(extract_args_module_token(&res));
                res.clear();
            }
            is_open = !is_open
        } else {
            res.push(ch);
        }
    }

    ret
}

fn extract_args_module_token(s: &String) -> Token {
    if !s.starts_with(":") {
        return Token::Module {
            name: s.trim().to_string(),
            opts: None,
        };
    } else {
        // TODO: fix unwraps
        let (name, args) = s.strip_prefix(":").unwrap().split_once(":").unwrap();
        let args_vec: Vec<String> = args.split(";").map(|s| s.to_string()).collect();
        return Token::Module {
            name: name.to_string(),
            opts: Some(args_vec),
        };
    }
}
