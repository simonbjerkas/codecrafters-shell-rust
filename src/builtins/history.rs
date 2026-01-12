use anyhow::Result;

use std::fs;

use super::{ShellCommand, writer};

const HISTORY_PATH: &'static str = "history.txt";

#[derive(Debug)]
pub struct History;

impl ShellCommand for History {
    fn name(&self) -> &'static str {
        "history"
    }

    fn execute(&self, _args: &Vec<String>) -> Result<Option<String>> {
        let hist = fs::read_to_string(HISTORY_PATH)?;
        println!("{}", hist);
        Ok(None)
    }
}

pub fn handle_history(line: &str) -> Result<()> {
    writer::write_file(HISTORY_PATH, line, &true)?;
    Ok(())
}
