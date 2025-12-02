use super::Commands;

use std::os::unix::fs::MetadataExt;

use rustyline::{
    completion::Completer, highlight::Highlighter, hint::Hinter, validate::Validator, Helper,
};

pub struct AutoCompleter;

impl Helper for AutoCompleter {}

impl Highlighter for AutoCompleter {}

impl Hinter for AutoCompleter {
    type Hint = &'static str;
}

impl Validator for AutoCompleter {}

impl Completer for AutoCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let last_space = &line
            .chars()
            .rev()
            .collect::<String>()
            .find(|c| c == ' ')
            .unwrap_or(pos);

        let partial = &line[pos - last_space..pos];

        let cmds: Vec<String> = Commands::all_commands()
            .iter()
            .filter(|cmd| cmd.starts_with(partial))
            .map(|cmd| cmd.to_string())
            .collect();

        if cmds.is_empty() {
            let cmds = search_executables(partial);
            return Ok((pos - last_space, cmds));
        }

        Ok((pos - last_space, cmds))
    }
}

pub fn search_executables(partial: &str) -> Vec<String> {
    let mut possibilities = Vec::new();
    let key = "PATH";
    if let Some(paths) = std::env::var_os(key) {
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
    }

    possibilities
}
