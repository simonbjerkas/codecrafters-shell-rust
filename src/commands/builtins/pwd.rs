use crate::{
    commands::{ShellCommand, ShellError},
    parser::ParsedInput,
};
use std::env;

pub struct Pwd;

impl ShellCommand for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn run(&self, input: &ParsedInput) -> Result<Option<String>, ShellError> {
        if !input.args.is_empty() {
            return Err(ShellError::Execution(format!(
                "{}: too many arguments",
                self.name()
            )));
        }
        if let Ok(current_dir) = env::current_dir() {
            return Ok(Some(format!("{}", &current_dir.display())));
        }
        Err(ShellError::Execution(format!(
            "Could not find current directory"
        )))
    }
}
