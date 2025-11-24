use crate::{
    commands::{ShellCommand, ShellError},
    parser::handle_res,
};
use std::env;

pub struct Pwd;

impl ShellCommand for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn run(&self, args: &[String]) -> Result<(), ShellError> {
        if !args.is_empty() {
            return Err(ShellError::Execution(format!(
                "{}: too many arguments",
                self.name()
            )));
        }
        if let Ok(current_dir) = env::current_dir() {
            if let Err(e) = handle_res(&current_dir.display().to_string(), args) {
                return Err(e);
            }

            return Ok(());
        }
        Err(ShellError::Execution(format!(
            "Could not find current directory"
        )))
    }
}
