use crate::parser::handle_res;

use super::{ShellCommand, ShellError};

pub struct Echo;

impl ShellCommand for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn run(&self, args: &[String]) -> Result<(), ShellError> {
        let display_string = args.join(" ");

        if let Err(e) = handle_res(&display_string, args) {
            return Err(e);
        }

        Ok(())
    }
}
