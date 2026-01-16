use anyhow::Result;

use super::{ExecResult, ShellCommand, ShellCtx};

#[derive(Debug)]
pub struct Echo;

impl ShellCommand for Echo {
    fn name(&self) -> &'static str {
        "echo"
    }

    fn execute(&self, args: &Vec<String>, _ctx: &mut ShellCtx) -> Result<ExecResult> {
        let display_string = format!("{}\n", args.join(" "));
        Ok(ExecResult::Res(display_string))
    }
}
