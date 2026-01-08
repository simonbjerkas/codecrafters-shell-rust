use anyhow::Result;
use std::env;

use super::{ShellCommand, ShellError};

#[derive(Debug)]
pub struct Cd;

impl ShellCommand for Cd {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn execute(&self, args: Vec<String>) -> Result<Option<String>> {
        if args.len() > 1 {
            return Err(
                ShellError::Execution(format!("{}: too many arguments", self.name())).into(),
            );
        }

        let directory = args.first();

        match directory {
            Some(dir) => {
                if dir.starts_with('~') {
                    if let Some(home) = env::home_dir() {
                        let path =
                            dir.replace('~', home.to_str().expect("Could not find home directory"));
                        if let Err(_) = env::set_current_dir(path) {
                            return Err(ShellError::Execution(format!(
                                "cd: {}; No such file or directory",
                                dir
                            ))
                            .into());
                        }
                    }
                } else {
                    if let Err(_) = env::set_current_dir(dir) {
                        return Err(ShellError::Execution(format!(
                            "cd: {}: No such file or directory",
                            dir
                        ))
                        .into());
                    }
                }
            }
            None => env::set_current_dir(env::home_dir().unwrap())
                .expect("Could not find home directory"),
        }

        Ok(None)
    }
}
