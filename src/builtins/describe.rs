use anyhow::Result;

use super::{super::Cmds, ShellCommand};

#[derive(Debug)]
pub struct Describe;

impl ShellCommand for Describe {
    fn name(&self) -> &'static str {
        "type"
    }

    fn execute(&self, args: &Vec<String>) -> Result<Option<String>> {
        if let Some(cmd_to_evaluate) = args.first() {
            let description = match Cmds::new(cmd_to_evaluate) {
                Cmds::Builtin(cmd) => cmd.description(),
                Cmds::External(cmd) => cmd.description(),
            };

            return Ok(Some(description));
        }
        Ok(None)
    }
}
