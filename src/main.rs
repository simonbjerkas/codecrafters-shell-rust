mod lexer;
mod parser;

use anyhow::Result;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let Some(tokens) = handle_result(lexer::run_lexer(&input)) else {
            continue;
        };

        println!("{tokens:?}");

        let Some((parsed, redirection)) = handle_result(parser::parse(tokens)) else {
            continue;
        };

        println!("********* {redirection:?}");

        if let Some(parsed) = parsed {
            if let Err(e) = codecrafters_shell::run_cmd(parsed.cmd, parsed.args, redirection) {
                eprintln!("{e}");
            }
        }
    }
}

fn handle_result<T>(data: Result<T>) -> Option<T> {
    match data {
        Ok(data) => return Some(data),
        Err(e) => {
            eprintln!("{e}");
            return None;
        }
    }
}
