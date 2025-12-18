use anyhow::Result;
use std::{env, path, process};

use super::{ShellCommand, ShellError, is_executable};

pub struct External {
    cmd: String,
}

impl External {
    pub fn new(cmd: String) -> External {
        External { cmd }
    }
}

impl ShellCommand for External {
    fn name(&self) -> &str {
        &self.cmd
    }

    fn description(&self) -> String {
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

    fn execute(&self, args: Vec<String>) -> Result<Option<String>> {
        let key = "PATH";
        let paths = env::var(key)?;
        for path in env::split_paths(&paths) {
            let cmd_path = path.join(&self.name());
            if !path::Path::new(&cmd_path).exists() || !is_executable(&cmd_path) {
                continue;
            };

            let mut program = process::Command::new(self.name());
            let mut child = program
                .args(args.clone())
                .spawn()
                .map_err(|e| ShellError::Execution(e.to_string()))?;
            let _status = child.wait();
            return Ok(None);
        }

        Err(ShellError::Execution(format!("{}: command not found", self.name())).into())
    }
}
