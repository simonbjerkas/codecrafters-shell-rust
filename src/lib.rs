use std::{
    fs,
    io::Write,
    os::unix::fs::MetadataExt,
    process::{Child, ChildStdout, Command, Stdio},
    thread,
};

mod builtins;
mod context;
mod error;
mod external;
mod writer;

pub mod redirection;

use anyhow::Result;
use external::External;

pub use builtins::{Builtins, ExecResult, ShellCommand};
pub use context::ShellCtx;
pub use error::ShellError;
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

pub fn execute_pipeline(parsed: ParsedLine, ctx: &mut ShellCtx) -> Result<ExecResult> {
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
                    let result = cmd.execute(args, ctx);

                    for mut c in children {
                        let _ = c.wait();
                    }
                    return handle_builtin_redirection(redirects, result);
                } else {
                    let mut out_buf: Vec<u8> = Vec::new();
                    let data = cmd.execute(args, ctx);
                    let result = handle_builtin_redirection(redirects, data)?;

                    if let ExecResult::Res(res) = result {
                        out_buf.extend_from_slice(res.as_bytes());
                    }

                    input_buf = Some(Buf::Builtin(out_buf));
                }
            }

            Cmds::External(cmd) => {
                let mut cmd = cmd.build(args)?;

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

                                if let Some(mut child_stdin) = child.stdin.take() {
                                    thread::spawn(move || {
                                        let _ = child_stdin.write_all(&buf);
                                    });
                                }

                                children.push(child);
                            }
                        }

                        if !is_last {
                            let last = children.pop().unwrap();
                            input_buf = if last.stdout.is_some() {
                                Some(Buf::External(last.stdout.unwrap()))
                            } else {
                                None
                            }
                        }
                    }

                    None => {
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

    for mut c in children {
        let _ = c.wait();
    }
    Ok(ExecResult::Continue)
}

fn handle_builtin_redirection(
    redirects: &Vec<Redirection>,
    data: Result<ExecResult>,
) -> Result<ExecResult> {
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
                        if let ExecResult::Res(res) = res
                            && !redirect_out
                        {
                            return Ok(ExecResult::Res(res.to_string()));
                        }
                    }
                    Err(e) => writer::write_file(path, e.to_string().as_str(), &append)?,
                }
            }
            redirection::Redirect::StdOut(append) => {
                writer::create_file(path, &append)?;
                match &data {
                    Ok(ExecResult::Res(res)) => {
                        writer::write_file(path, res.clone().as_ref(), &append)?
                    }
                    Ok(_) => writer::write_file(path, "", &append)?,
                    Err(e) => {
                        if !redirect_err {
                            return Err(ShellError::Execution(e.to_string()).into());
                        }
                    }
                }
            }
        }
    }
    Ok(ExecResult::Continue)
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
