use anyhow::Result;

use std::{
    fs,
    io::{self, BufRead},
};

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
        println!("{}", hist.trim());
        Ok(None)
    }
}

pub fn handle_history(line: &str) -> Result<()> {
    let file = fs::File::open(HISTORY_PATH);
    let line_count = match file {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            let lines = reader.lines();

            lines.count() + 1
        }
        Err(_) => 1,
    };

    let line = format!("    {} {}", line_count, line);

    writer::write_file(HISTORY_PATH, line.as_str(), &true)?;
    Ok(())
}
