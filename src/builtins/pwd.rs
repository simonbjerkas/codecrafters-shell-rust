use anyhow::Result;
use std::env;

use super::{ShellCommand, ShellError};

#[derive(Debug)]
pub struct Pwd;

impl ShellCommand for Pwd {
    fn name(&self) -> &'static str {
        "pwd"
    }

    fn execute(&self, args: Vec<String>) -> Result<Option<String>> {
        if !args.is_empty() {
            return Err(
                ShellError::Execution(format!("{}: too many arguments", self.name())).into(),
            );
        }
        if let Ok(current_dir) = env::current_dir() {
            return Ok(Some(format!("{}", &current_dir.display())));
        }
        Err(ShellError::Execution(format!("Could not find current directory")).into())
    }
}
