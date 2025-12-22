use anyhow::Result;

use super::{super::Commands, ShellCommand};

pub struct Describe;

impl ShellCommand for Describe {
    fn name(&self) -> &'static str {
        "type"
    }

    fn execute(&self, args: Vec<String>) -> Result<Option<String>> {
        if let Some(cmd_to_evaluate) = args.first() {
            let description = match Commands::new(cmd_to_evaluate) {
                Commands::Builtin(cmd) => cmd.description(),
                Commands::External(cmd) => cmd.description(),
            };

            return Ok(Some(description));
        }
        Ok(None)
    }
}
