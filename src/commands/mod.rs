mod builtins;
pub mod enums;

use super::{parser::ParsedInput, ShellError};

use std::{fs, os::unix::fs::MetadataExt};

pub trait ShellCommand {
    fn name(&self) -> &str;
    fn description(&self) -> String {
        format!("{} is a shell builtin", self.name())
    }
    fn run(&self, input: &ParsedInput) -> Result<Option<String>, ShellError>;
}

pub fn is_executable(path: &std::path::Path) -> bool {
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
