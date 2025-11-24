mod commands;
mod error;
mod parser;

use commands::enums::Commands;
use error::ShellError;

use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let (command, args) = parser::parse(&input);

        if let Some(cmd) = Commands::from_cmd(&command) {
            if let Err(_) = cmd.run(&args) {
                eprintln!("Oh now!")
            }
        }
    }
}
