use anyhow::Result;

use super::ShellCommand;

#[derive(Debug)]
pub struct History;

impl ShellCommand for History {
    fn name(&self) -> &'static str {
        "history"
    }

    fn execute(&self, _args: &Vec<String>) -> Result<Option<String>> {
        todo!()
    }
}
