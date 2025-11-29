use std::{env, io::Read, path, process};

use super::is_executable;
use crate::{
    commands::ShellCommand,
    error::ShellError,
    parser::{OutputStyle, ParsedInput},
};

pub struct Unknown {
    cmd: String,
}

impl Unknown {
    pub fn new(cmd: String) -> Self {
        Self { cmd }
    }
}

impl ShellCommand for Unknown {
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

    fn run(&self, input: &ParsedInput) -> Result<Option<String>, super::ShellError> {
        let key = "PATH";
        if let Some(paths) = env::var_os(key) {
            for path in env::split_paths(&paths) {
                let cmd_path = path.join(&self.name());
                if path::Path::new(&cmd_path).exists() & is_executable(&cmd_path) {
                    let mut program = process::Command::new(self.name());
                    match input.output {
                        OutputStyle::Print => {
                            let mut child = program
                                .args(input.args.clone())
                                .spawn()
                                .map_err(|e| ShellError::Execution(e.to_string()))?;
                            let _status = child.wait();
                            return Ok(None);
                        }
                        OutputStyle::StdOut { .. } => {
                            let mut child = program
                                .args(input.args.clone())
                                .stdout(process::Stdio::piped())
                                .spawn()
                                .map_err(|e| ShellError::Execution(e.to_string()))?;
                            let _status = child.wait();

                            if let Some(mut stdout) = child.stdout.take() {
                                let mut output = String::new();
                                stdout
                                    .read_to_string(&mut output)
                                    .map_err(|e| ShellError::Execution(e.to_string()))?;
                                return Ok(Some(output));
                            }
                            return Ok(None);
                        }
                        OutputStyle::StdErr { .. } => {
                            let mut child = program
                                .args(input.args.clone())
                                .stderr(process::Stdio::piped())
                                .spawn()
                                .map_err(|e| ShellError::Execution(e.to_string()))?;
                            let _status = child.wait();

                            if let Some(mut stderr) = child.stderr.take() {
                                let mut error = String::new();
                                stderr
                                    .read_to_string(&mut error)
                                    .map_err(|e| ShellError::Execution(e.to_string()))?;
                                return Ok(Some(error));
                            }
                            return Ok(None);
                        }
                    }
                }
            }
        }

        Err(ShellError::Execution(format!(
            "{}: command not found",
            self.name()
        )))
    }
}
