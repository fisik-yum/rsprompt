use std::fmt;
use std::fmt::Display;

use crate::matcher::match_token;
pub enum Token<'a> {
    Text(&'a str),
    Module {
        name: &'a str,
        opts: Option<Vec<&'a str>>,
    },
}

impl<'a> Display for Token<'a> {
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
// TODO: nesting
fn tokenize<'a>(dat: &'a str) -> Vec<Token<'a>> {
    let mut ret: Vec<Token<'a>> = vec![];

    let mut is_open = false;
    let mut segment_start = 0;

    for (i, ch) in dat.char_indices() {
        if ch == '{' {
            if !is_open {
                if i > segment_start {
                    ret.push(Token::Text(&dat[segment_start..i]));
                }
                is_open = true;
                segment_start = i + 1;
            }
        } else if ch == '}' {
            if is_open {
                is_open = false;
                ret.push(extract_args_module_token(&dat[segment_start..i]));
                segment_start = i + 1;
            }
        }
    }
    ret
}

fn extract_args_module_token<'a>(s: &'a str) -> Token<'a> {
    if !s.starts_with(":") {
        return Token::Module {
            name: s.trim(),
            opts: None,
        };
    } else {
        // TODO: fix unwraps
        let (name, args) = s.strip_prefix(":").unwrap().split_once(":").unwrap();
        let args_vec: Vec<&'a str> = args.split(";").map(|s| s).collect();
        return Token::Module {
            name: name,
            opts: Some(args_vec),
        };
    }
}
