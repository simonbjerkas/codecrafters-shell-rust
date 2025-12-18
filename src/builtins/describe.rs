use anyhow::Result;

use super::{Commands, ShellCommand};

pub struct Describe;

impl ShellCommand for Describe {
    fn name(&self) -> &'static str {
        "type"
    }

    fn execute(&self, args: Vec<String>) -> Result<Option<String>> {
        if let Some(cmd_to_evaluate) = args.first() {
            let evaluated_cmd = Commands::new(cmd_to_evaluate);
            return Ok(Some(evaluated_cmd.description()));
        }
        Ok(None)
    }
}
