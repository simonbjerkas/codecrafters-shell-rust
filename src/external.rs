use anyhow::{Ok, Result};
use std::{
    env, path,
    process::{self, Command},
};

use super::{ShellError, is_executable};

#[derive(Debug)]
pub struct External {
    cmd: String,
}

impl External {
    pub fn new(cmd: String) -> External {
        External { cmd }
    }
}

impl External {
    fn name(&self) -> &str {
        &self.cmd
    }

    pub fn description(&self) -> String {
        let key = "PATH";
        if let Some(paths) = env::var_os(key) {
            for path in env::split_paths(&paths) {
                let cmd_path = path.join(self.name());
                if path::Path::new(&cmd_path).exists() & is_executable(&cmd_path) {
                    return format!("{} is {}", self.name(), cmd_path.display());
                }
            }
        }

        format!("{}: not found", self.name())
    }

    pub fn build(&self, args: &Vec<String>) -> Result<Command> {
        let mut program = build_command(self.name())?;
        program.args(args);

        Ok(program)
    }
}

fn build_command(cmd: &str) -> Result<Command> {
    let key = "PATH";
    let paths = env::var(key)?;
    for path in env::split_paths(&paths) {
        let cmd_path = path.join(cmd);
        if !path::Path::new(&cmd_path).exists() || !is_executable(&cmd_path) {
            continue;
        };

        return Ok(process::Command::new(cmd));
    }

    Err(ShellError::Execution(format!("{cmd}: command not found")).into())
}
