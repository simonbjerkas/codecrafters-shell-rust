use std::{
    fs,
    io::Write,
    os::unix::fs::MetadataExt,
    process::{Child, ChildStdout, Command, Stdio},
    thread,
};

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

#[derive(Debug)]
pub enum Cmds {
    Builtin(Box<dyn ShellCommand>),
    External(External),
}

impl Cmds {
    pub fn new(cmd: &str) -> Cmds {
        match Builtins::new(cmd) {
            Some(cmd) => Cmds::Builtin(cmd),
            None => Cmds::External(External::new(cmd.to_string())),
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

pub fn search_executables(partial: &str) -> Result<Vec<String>> {
    let mut possibilities = Vec::new();
    let key = "PATH";
    let paths = std::env::var(key)?;

    for path in std::env::split_paths(&paths) {
        let Some(dirs) = std::fs::read_dir(path).ok() else {
            continue;
        };

        for f in dirs {
            let Some(f) = f.ok() else {
                continue;
            };
            let file_name = f.file_name().display().to_string();
            let is_executable = f
                .metadata()
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
                .unwrap_or(false);

            if file_name.starts_with(partial) && is_executable {
                possibilities.push(format!("{} ", f.file_name().display()));
            }
        }
    }

    Ok(possibilities)
}

pub enum ParsedLine<'a> {
    Pipeline(Vec<CommandStage<'a>>),
}

#[derive(Debug)]
pub struct CommandStage<'a> {
    pub cmd: Cmds,
    pub args: Vec<String>,
    pub redirects: Vec<Redirection<'a>>,
}

pub fn execute_pipeline(parsed: ParsedLine) -> Result<Option<String>> {
    let ParsedLine::Pipeline(pipeline) = parsed;

    enum Buf {
        External(ChildStdout),
        Builtin(Vec<u8>),
    }

    let mut input_buf: Option<Buf> = None;
    let mut children: Vec<Child> = Vec::new();

    for (i, stage) in pipeline.iter().enumerate() {
        let is_last = i + 1 == pipeline.len();
        let CommandStage {
            cmd,
            args,
            redirects,
        } = stage;

        match cmd {
            Cmds::Builtin(cmd) => {
                if is_last {
                    // Last stage: write directly to terminal/stdout
                    let result = cmd.execute(args);
                    // Wait for any earlier externals.
                    for mut c in children {
                        let _ = c.wait();
                    }
                    return handle_builtin_redirection(redirects, result);
                } else {
                    // Middle stage: write into buffer for next stage
                    let mut out_buf: Vec<u8> = Vec::new();
                    let data = cmd.execute(args);
                    let result = handle_builtin_redirection(redirects, data)?;

                    if result.is_some() {
                        out_buf.extend_from_slice(result.unwrap().as_bytes());
                    }

                    input_buf = Some(Buf::Builtin(out_buf));
                }
            }

            Cmds::External(cmd) => {
                let mut cmd = cmd.build(args)?;

                // stdin
                match input_buf.take() {
                    Some(buf) => {
                        cmd.stdin(Stdio::piped());
                        if is_last {
                            cmd.stdout(Stdio::inherit());
                        } else {
                            cmd.stdout(Stdio::piped());
                        }

                        handle_external_redirection(&redirects, &mut cmd)?;

                        match buf {
                            Buf::External(buf) => {
                                cmd.stdin(Stdio::from(buf));
                                let child = cmd.spawn()?;
                                children.push(child);
                            }
                            Buf::Builtin(buf) => {
                                let mut child = cmd.spawn()?;

                                // Feed stdin with the buffer, then close it to signal EOF.
                                // If you worry about blocking on huge buffers, do this in a thread.
                                if let Some(mut child_stdin) = child.stdin.take() {
                                    // For large data, thread avoids deadlocks if child also writes a lot.
                                    thread::spawn(move || {
                                        let _ = child_stdin.write_all(&buf);
                                        // dropping child_stdin closes it
                                    });
                                }

                                children.push(child);
                            }
                        }

                        if !is_last {
                            // For a simple approach, wait, read output, store as input_buf.
                            // This loses streaming between externals but is simplest.
                            let last = children.pop().unwrap();
                            input_buf = if last.stdout.is_some() {
                                Some(Buf::External(last.stdout.unwrap()))
                            } else {
                                None
                            }
                        }
                    }

                    None => {
                        // stdin from shell or from previous external if we connected pipes.
                        // Here we choose a direct streaming approach between externals:
                        // - if this isn't the first stage and we have no input_buf, that means
                        //   the previous stage was an external and we should have set input_buf.
                        // So: if None, we use inherited stdin (shell).
                        cmd.stdin(Stdio::inherit());

                        if is_last {
                            cmd.stdout(Stdio::inherit());

                            handle_external_redirection(&redirects, &mut cmd)?;

                            let child = cmd.spawn()?;
                            children.push(child);
                        } else {
                            cmd.stdout(Stdio::piped());

                            handle_external_redirection(&redirects, &mut cmd)?;

                            let child = cmd.spawn()?;
                            input_buf = if child.stdout.is_some() {
                                Some(Buf::External(child.stdout.unwrap()))
                            } else {
                                None
                            }
                        }
                    }
                }
            }
        }
    }

    // If we fall through, last command was external but we buffered; wait children.
    for mut c in children {
        let _ = c.wait();
    }
    Ok(None)
}

fn handle_builtin_redirection(
    redirects: &Vec<Redirection>,
    data: Result<Option<String>>,
) -> Result<Option<String>> {
    if redirects.is_empty() {
        return data;
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
                match &data {
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
                match &data {
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

fn handle_external_redirection(redirects: &Vec<Redirection>, cmd: &mut Command) -> Result<()> {
    if redirects.is_empty() {
        return Ok(());
    }
    for redirect in redirects {
        match redirect.redirect {
            Redirect::StdOut(append) => {
                let file = writer::create_file(redirect.path, &append)?;
                cmd.stdout(Stdio::from(file));
            }
            Redirect::StdErr(append) => {
                let file = writer::create_file(redirect.path, &append)?;
                cmd.stderr(Stdio::from(file));
            }
        }
    }
    Ok(())
}
