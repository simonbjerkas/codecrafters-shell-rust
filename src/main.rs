mod commands;
mod error;
mod parser;
mod rlhelper;
mod writer;

use commands::enums::Commands;
use error::ShellError;

use std::io::{self, Write};

fn main() {
    let mut rl = rustyline::Editor::new().unwrap();
    rl.set_helper(Some(rlhelper::AutoCompleter));
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let readline = rl.readline("$ ");

        match readline {
            Ok(input) => {
                let Some(input) = parser::parse(&input) else {
                    continue;
                };
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
                        Ok(content) => match content {
                            Some(content) => {
                                if let Err(e) = writer::write_file(&path, &content, &append) {
                                    eprintln!("{e}")
                                }
                            }
                            None => {
                                if let Err(e) = writer::create_file(&path, &append) {
                                    eprintln!("{}", e)
                                }
                            }
                        },
                        Err(e) => eprintln!("{}", e),
                    },
                    parser::OutputStyle::StdErr { path, append } => match output {
                        Ok(content) => {
                            if let Some(content) = content {
                                println!("{}", content);
                            }
                            if let Err(e) = writer::create_file(&path, &append) {
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
            Err(rustyline::error::ReadlineError::Interrupted) => break,
            Err(e) => eprintln!("{}", e),
        }
    }
}
