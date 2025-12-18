use anyhow::Result;
use std::{fs, os::unix::fs::MetadataExt};

mod builtins;
mod error;
mod external;
pub mod redirection;
mod writer;

use builtins::*;
pub use error::ShellError;
use external::External;
pub use redirection::Redirection;

pub struct Commands;

impl Commands {
    pub fn new(cmd: &str) -> Box<dyn ShellCommand> {
        match cmd {
            "exit" => Box::new(Exit),
            "echo" => Box::new(Echo),
            "pwd" => Box::new(Pwd),
            "type" => Box::new(Describe),
            "cd" => Box::new(Cd),
            cmd => Box::new(External::new(cmd.to_string())),
        }
    }

    pub fn all_commands() -> Vec<&'static str> {
        Vec::from(["exit", "echo", "pwd", "type", "cd"])
    }
}

pub trait ShellCommand {
    fn name(&self) -> &str;
    fn description(&self) -> String {
        format!("{} is a shell builtin", self.name())
    }
    fn execute(&self, args: Vec<String>) -> Result<Option<String>>;
}

fn is_executable(path: &std::path::Path) -> bool {
    fs::metadata(path)
        .map(|metadata| {
            let mode = metadata.mode();
            let owner_executable = (mode & 0o100) != 0;
            let group_executable = (mode & 0o010) != 0;
            let others_executable = (mode & 0o001) != 0;
            if owner_executable || group_executable || others_executable {
                true
            } else {
                false
            }
        })
        .unwrap_or(false)
}

pub fn run_cmd(
    cmd: Box<dyn ShellCommand>,
    args: Vec<String>,
    redirect: Option<Redirection>,
) -> Result<()> {
    let result = cmd.execute(args);

    let Some(redirect) = redirect else {
        match result {
            Ok(res) => {
                if let Some(res) = res {
                    println!("{res}");
                }
            }
            Err(e) => eprintln!("{e}"),
        }
        return Ok(());
    };

    let Redirection { redirect, path } = redirect;

    match redirect {
        redirection::Redirect::StdErr(append) => match result {
            Ok(res) => {
                if let Some(res) = res {
                    println!("{res}");
                }
            }
            Err(e) => writer::write_file(path, e.to_string().as_str(), &append)?,
        },
        redirection::Redirect::StdOut(append) => match result {
            Ok(res) => writer::write_file(path, &res.unwrap_or(String::new()), &append)?,
            Err(e) => {
                eprintln!("{e}");
            }
        },
    }

    Ok(())
}
