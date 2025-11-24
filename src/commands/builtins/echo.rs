use crate::parser::handle_res;

use super::{ShellCommand, ShellError};

pub struct Echo;

impl ShellCommand for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, args: &[String]) -> Result<(), ShellError> {
        let display_string = args;

        for line in display_string {
            if let Err(e) = handle_res(line, args) {
                return Err(e);
            }
        }

        Ok(())
    }
}
