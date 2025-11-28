use crate::{
    commands::{ShellCommand, ShellError},
    parser::ParsedInput,
    Commands,
};

pub struct Describe;

impl ShellCommand for Describe {
    fn name(&self) -> &'static str {
        "type"
    }

    fn run(&self, input: &ParsedInput) -> Result<Option<String>, ShellError> {
        if let Some(cmd_to_evaluate) = input.args.first() {
            let evaluated_cmd = Commands::from_cmd(cmd_to_evaluate);
            return Ok(Some(evaluated_cmd.type_description()));
        }
        Ok(None)
    }
}
