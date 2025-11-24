mod builtins;
pub mod enums;

use super::parser::handle_res;
use super::ShellError;

pub trait ShellCommand {
    fn name(&self) -> &'static str;
    fn description(&self) -> String {
        format!("{} is a shell builtin", self.name())
    }
    fn run(&self, args: &[String]) -> Result<(), ShellError>;
}
