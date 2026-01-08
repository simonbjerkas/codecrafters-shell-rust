use anyhow::Result;
use std::{
    env, path,
    process::{self, Stdio},
};

use super::{Redirect, Redirection, ShellError, is_executable, writer};

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

    pub fn execute_external(&self, args: Vec<String>, redirects: Vec<Redirection>) -> Result<()> {
        let key = "PATH";
        let paths = env::var(key)?;
        for path in env::split_paths(&paths) {
            let cmd_path = path.join(&self.name());
            if !path::Path::new(&cmd_path).exists() || !is_executable(&cmd_path) {
                continue;
            };

            let mut program = process::Command::new(self.name());
            program.args(args);

            let mut stdout = Stdio::inherit();
            let mut stderr = Stdio::inherit();

            for redirect in redirects {
                match redirect.redirect {
                    Redirect::StdOut(append) => {
                        let file = writer::create_file(redirect.path, &append)?;
                        stdout = Stdio::from(file);
                    }
                    Redirect::StdErr(append) => {
                        let file = writer::create_file(redirect.path, &append)?;
                        stderr = Stdio::from(file);
                    }
                }
            }

            program.stdout(stdout);
            program.stderr(stderr);

            let mut child = program.spawn()?;
            child.wait()?;

            return Ok(());
        }

        Err(ShellError::Execution(format!("{}: command not found", self.name())).into())
    }
}
