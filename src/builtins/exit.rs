use anyhow::Result;

use super::{ExecResult, ShellCommand, ShellCtx};

#[derive(Debug)]
pub struct Exit;

impl ShellCommand for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn execute(&self, args: &Vec<String>, ctx: &mut ShellCtx) -> Result<ExecResult> {
        let status = args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0);

        ctx.shut_down()?;
        Ok(ExecResult::Exit(status))
    }
}
