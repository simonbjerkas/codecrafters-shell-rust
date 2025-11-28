use std::process;

use crate::{
    commands::{ShellCommand, ShellError},
    parser::ParsedInput,
};

pub struct Exit;

impl ShellCommand for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn run(&self, input: &ParsedInput) -> Result<Option<String>, ShellError> {
        let code = input.args.iter().next();

        match code {
            Some(status) => {
                let status = status.parse::<i32>();

                match status {
                    Ok(status) => process::exit(status),
                    Err(_) => process::exit(0),
                }
            }
            None => process::exit(0),
        }
    }
}
