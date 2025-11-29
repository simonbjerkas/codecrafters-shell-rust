mod commands;
mod error;
mod parser;
mod writer;

use commands::enums::Commands;
use error::ShellError;

use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Some(input) = parser::parse(&input) {
            let output = input.cmd.run(&input);
            match input.output {
                parser::OutputStyle::Print => match output {
                    Ok(content) => {
                        if let Some(content) = content {
                            println!("{}", content)
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                },
                parser::OutputStyle::StdOut { path, append } => match output {
                    Ok(content) => {
                        if let Some(content) = content {
                            if let Err(e) = writer::write_file(&path, &content, &append) {
                                eprintln!("{e}")
                            }
                        }
                    }
                    Err(e) => eprintln!("{}", e),
                },
                parser::OutputStyle::StdErr { path, append } => match output {
                    Ok(content) => {
                        if let Some(content) = content {
                            println!("{}", content);
                        }
                        if let Err(e) = writer::create_file(&path) {
                            eprintln!("{}", e)
                        }
                    }
                    Err(e) => {
                        if let Err(e) = writer::write_file(&path, &e.to_string(), &append) {
                            eprintln!("{}", e)
                        }
                    }
                },
            }
        }
    }
}
