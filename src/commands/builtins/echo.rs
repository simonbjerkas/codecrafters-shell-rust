use crate::parser::ParsedInput;

use super::{ShellCommand, ShellError};

pub struct Echo;

impl ShellCommand for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, input: &ParsedInput) -> Result<Option<String>, ShellError> {
        let display_string = input.args.join(" ");
        Ok(Some(display_string))
    }
}
