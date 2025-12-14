mod commands;
mod error;
mod lexer;
mod parser;
mod rlhelper;
mod writer;

use commands::enums::Commands;
use error::ShellError;

use std::io::{self, Write};

use crate::parser::ParsedInput;

fn main() {
    let config = rustyline::Config::builder()
        .completion_type(rustyline::CompletionType::List)
        .build();

    let mut rl = rustyline::Editor::with_config(config).unwrap();
    rl.set_helper(Some(rlhelper::AutoCompleter));
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let readline = rl.readline("$ ");

        match readline {
            Ok(input) => {
                let mut cmds = parser::parse(&input);
                let cmd = cmds.remove(0);
                run_cmd(cmd, &mut cmds);
            }
            Err(rustyline::error::ReadlineError::Interrupted) => break,
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn run_cmd(cmd: Option<ParsedInput>, cmds: &mut Vec<Option<ParsedInput>>) {
    let Some(input) = cmd else { return };

    let output = input.cmd.run(&input);

    match input.output {
        parser::OutputStyle::Print => match output {
            Ok(content) => {
                if let Some(content) = content {
                    if cmds.is_empty() {
                        println!("{}", content);
                    } else {
                        let Some(mut next_cmd) = cmds.remove(0) else {
                            eprintln!("Pipe failed");
                            return;
                        };

                        next_cmd.args.push(content);

                        run_cmd(Some(next_cmd), cmds);
                    }
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
