use anyhow::Result;

use super::ShellCommand;

pub struct Echo;

impl ShellCommand for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn execute(&self, args: Vec<String>) -> Result<Option<String>> {
        let display_string = format!("{}", args.join(" "));
        Ok(Some(display_string))
    }
}
