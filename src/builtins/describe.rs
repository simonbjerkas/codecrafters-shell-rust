use anyhow::Result;

use super::{super::Cmds, ExecResult, ShellCommand, ShellCtx};

#[derive(Debug)]
pub struct Describe;

impl ShellCommand for Describe {
    fn name(&self) -> &'static str {
        "type"
    }

    fn execute(&self, args: &Vec<String>, _ctx: &mut ShellCtx) -> Result<ExecResult> {
        if let Some(cmd_to_evaluate) = args.first() {
            let description = match Cmds::new(cmd_to_evaluate) {
                Cmds::Builtin(cmd) => cmd.description(),
                Cmds::External(cmd) => cmd.description(),
            };

            return Ok(ExecResult::Res(description));
        }
        Ok(ExecResult::Continue)
    }
}
