use anyhow::Result;

use super::{ShellCommand, ShellCtx};

#[derive(Debug)]
pub struct Echo;

impl ShellCommand for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn execute(&self, args: &Vec<String>, _ctx: &mut ShellCtx) -> Result<Option<String>> {
        let display_string = format!("{}\n", args.join(" "));
        Ok(Some(display_string))
    }
}
