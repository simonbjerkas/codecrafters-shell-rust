mod commands;
mod error;
mod parser;
mod writer;

use commands::enums::Commands;
use error::ShellError;
use writer::write_file;

use std::io::{self, Write};

use crate::parser::OutputStyle;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let parsed_input = parser::parse(&input);

        match parsed_input {
            Some(input) => {
                let output = input.cmd.run(&input);
                match input.output {
                    OutputStyle::Print => match output {
                        Ok(content) => {
                            if let Some(content) = content {
                                println!("{}", content)
                            }
                        }
                        Err(e) => eprintln!("{}", e),
                    },
                    OutputStyle::StdOut { path } => match output {
                        Ok(content) => {
                            if let Some(content) = content {
                                if let Err(e) = write_file(&path, content) {
                                    eprintln!("{e}")
                                }
                            }
                        }
                        Err(e) => {
                            if let Err(e) = write_file(&path, e.to_string()) {
                                eprintln!("{e}")
                            }
                        }
                    },
                }
            }
            None => {
                println!("");
            }
        }
    }
}
