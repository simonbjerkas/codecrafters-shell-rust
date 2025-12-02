use super::Commands;

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
    type Candidate = &'static str;

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

        let cmds = Commands::all_commands()
            .iter()
            .filter(|cmd| cmd.starts_with(&line[pos - last_space..pos]))
            .copied()
            .collect();

        Ok((pos - last_space, cmds))
    }
}
