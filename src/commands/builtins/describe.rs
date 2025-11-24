use crate::{
    commands::{ShellCommand, ShellError},
    Commands,
};

use super::handle_res;

pub struct Describe;

impl ShellCommand for Describe {
    fn name(&self) -> &'static str {
        "type"
    }

    fn run(&self, args: &[String]) -> Result<(), ShellError> {
        if let Some(cmd_to_evaluate) = args.first() {
            if let Some(evaluated_cmd) = Commands::from_cmd(cmd_to_evaluate) {
                if let Err(e) = handle_res(&evaluated_cmd.type_description(), args) {
                    return Err(e);
                }
                return Ok(());
            }
            return Err(ShellError::Execution(format!(
                "{} not found",
                cmd_to_evaluate
            )));
        }
        Ok(())
    }
}
