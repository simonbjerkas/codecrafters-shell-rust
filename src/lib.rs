use std::{fs, os::unix::fs::MetadataExt};

mod builtins;
mod error;
mod external;
pub mod redirection;
mod writer;

use anyhow::Result;
pub use builtins::{Builtins, ShellCommand};
pub use error::ShellError;
use external::External;
pub use redirection::{Redirect, Redirection};

pub enum Commands {
    Builtin(Box<dyn ShellCommand>),
    External(External),
}

impl Commands {
    pub fn new(cmd: &str) -> Commands {
        match Builtins::new(cmd) {
            Some(cmd) => Commands::Builtin(cmd),
            None => Commands::External(External::new(cmd.to_string())),
        }
    }
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
    cmd: Commands,
    args: Vec<String>,
    redirects: Vec<Redirection>,
) -> Result<Option<String>> {
    match cmd {
        Commands::Builtin(cmd) => {
            let result = cmd.execute(args);

            if redirects.is_empty() {
                match result {
                    Ok(res) => return Ok(res),
                    Err(e) => return Err(e),
                }
            };

            let redirect_out = redirects
                .iter()
                .any(|r| matches!(r.redirect, Redirect::StdOut(_)));
            let redirect_err = redirects
                .iter()
                .any(|r| matches!(r.redirect, Redirect::StdErr(_)));

            for redirect in redirects {
                let Redirection { redirect, path } = redirect;

                match redirect {
                    redirection::Redirect::StdErr(append) => {
                        writer::create_file(path, &append)?;
                        match &result {
                            Ok(res) => {
                                if let Some(res) = res
                                    && !redirect_out
                                {
                                    return Ok(Some(res.to_string()));
                                }
                            }
                            Err(e) => writer::write_file(path, e.to_string().as_str(), &append)?,
                        }
                    }
                    redirection::Redirect::StdOut(append) => {
                        writer::create_file(path, &append)?;
                        match &result {
                            Ok(res) => writer::write_file(
                                path,
                                res.clone().unwrap_or(String::new()).as_str(),
                                &append,
                            )?,
                            Err(e) => {
                                if !redirect_err {
                                    return Err(ShellError::Execution(e.to_string()).into());
                                }
                            }
                        }
                    }
                }
            }
            Ok(None)
        }
        Commands::External(cmd) => {
            cmd.execute_external(args, redirects)?;
            Ok(None)
        }
    }
}
