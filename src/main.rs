use std::env;
mod matcher;
mod mod_git;
mod mod_go;
mod mod_text;
mod modules;
mod token;
fn main() {
    if let Ok(prompt) = env::var("PROMPT") {
        let result = token::parse(&prompt);
        print!("{}", result)
    }
}
