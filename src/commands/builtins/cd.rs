use std::env;

use crate::{
    commands::{ShellCommand, ShellError},
    parser::ParsedInput,
};

pub struct Cd;

impl ShellCommand for Cd {
    fn name(&self) -> &'static str {
        "cd"
    }

    fn run(&self, input: &ParsedInput) -> Result<Option<String>, ShellError> {
        if input.args.len() > 1 {
            return Err(ShellError::Execution(format!(
                "{}: too many arguments",
                self.name()
            )));
        }

        let directory = input.args.first();

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
                            )));
                        }
                    }
                } else {
                    if let Err(_) = env::set_current_dir(dir) {
                        return Err(ShellError::Execution(format!(
                            "cd: {}: No such file or directory",
                            dir
                        )));
                    }
                }
            }
            None => env::set_current_dir(env::home_dir().unwrap())
                .expect("Could not find home directory"),
        }

        Ok(None)
    }
}
