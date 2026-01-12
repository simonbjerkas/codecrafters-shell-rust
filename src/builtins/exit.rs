use anyhow::Result;
use std::process;

use super::ShellCommand;

#[derive(Debug)]
pub struct Exit;

impl ShellCommand for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn execute(&self, args: &Vec<String>) -> Result<Option<String>> {
        let code = args.iter().next();

        match code {
            Some(status) => {
                let status = status.parse::<i32>();

                match status {
                    Ok(status) => process::exit(status),
                    Err(_) => process::exit(0),
                }
            }
            None => process::exit(0),
        }
    }
}
