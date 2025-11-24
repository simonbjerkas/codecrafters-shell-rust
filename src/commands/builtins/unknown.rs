use std::{env, fs, os::unix::fs::MetadataExt, path, process};

use crate::error::ShellError;

pub fn run(cmd: &str, args: &[String]) -> Result<(), super::ShellError> {
    let key = "PATH";
    if let Some(paths) = env::var_os(key) {
        for path in env::split_paths(&paths) {
            let cmd_path = path.join(&cmd);
            if path::Path::new(&cmd_path).exists() & is_executable(&cmd_path) {
                let mut program = process::Command::new(&cmd);

                let mut process = program
                    .args(args)
                    .spawn()
                    .expect("Failed to execute command");

                let _status = process.wait().expect("Something went wrong");
                return Ok(());
            }
        }
    }

    Err(ShellError::Execution(format!("{}: command not found", cmd)))
}

pub fn is_executable(path: &path::Path) -> bool {
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
