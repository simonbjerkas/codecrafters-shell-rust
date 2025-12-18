use anyhow::Result;
use std::{env, io::Read, path, process};

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
        // if let Some(paths) = env::var_os(key) {
        //     for path in env::split_paths(&paths) {
        //         let cmd_path = path.join(&self.name());
        //         if path::Path::new(&cmd_path).exists() & is_executable(&cmd_path) {
        //             let mut program = process::Command::new(self.name());
        //             match input.output {
        //                 OutputStyle::Print => {
        //                     let mut child = program
        //                         .args(input.args.clone())
        //                         .spawn()
        //                         .map_err(|e| ShellError::Execution(e.to_string()))?;
        //                     let _status = child.wait();
        //                     return Ok(None);
        //                 }
        //                 OutputStyle::StdOut { .. } => {
        //                     let mut child = program
        //                         .args(input.args.clone())
        //                         .stdout(process::Stdio::piped())
        //                         .spawn()
        //                         .map_err(|e| ShellError::Execution(e.to_string()))?;
        //                     let _status = child.wait();

        //                     if let Some(mut stdout) = child.stdout.take() {
        //                         let mut output = String::new();
        //                         stdout
        //                             .read_to_string(&mut output)
        //                             .map_err(|e| ShellError::Execution(e.to_string()))?;

        //                         if output.is_empty() {
        //                             return Ok(None);
        //                         }
        //                         return Ok(Some(output));
        //                     }
        //                     return Ok(None);
        //                 }
        //                 OutputStyle::StdErr { .. } => {
        //                     let mut child = program
        //                         .args(input.args.clone())
        //                         .stderr(process::Stdio::piped())
        //                         .spawn()
        //                         .map_err(|e| ShellError::Execution(e.to_string()))?;
        //                     let _status = child.wait();

        //                     if let Some(mut stderr) = child.stderr.take() {
        //                         let mut error = String::new();
        //                         stderr
        //                             .read_to_string(&mut error)
        //                             .map_err(|e| ShellError::Execution(e.to_string()))?;
        //                         return Err(ShellError::Execution(error));
        //                     }
        //                     return Ok(None);
        //                 }
        //             }
        //         }
        //     }
        // }

        Err(ShellError::Execution(format!("{}: command not found", self.name())).into())
    }
}
