use std::{
    env, fs,
    io::{self, BufRead},
};

use super::writer;
use anyhow::Result;

#[derive(Default)]
pub struct ShellCtx {
    pub history: Vec<String>,
    histfile: Option<String>,
}

impl ShellCtx {
    pub fn build() -> Result<Self> {
        let mut ctx = ShellCtx {
            history: Vec::new(),
            histfile: None,
        };

        if let Ok(path) = env::var("HISTFILE") {
            ctx.set_histfile(path)?;
        };

        Ok(ctx)
    }

    pub fn set_histfile(&mut self, path: String) -> Result<()> {
        self.histfile = Some(path.clone());

        let file = fs::OpenOptions::new().create(true).open(path)?;
        let reader = io::BufReader::new(file);

        self.history = reader.lines().collect::<Result<_, _>>()?;

        Ok(())
    }

    pub fn handle_history(&self, line: &str) -> Result<()> {
        if let Some(histfile) = &self.histfile {
            writer::write_file(&histfile, line, &true)?;
        }
        Ok(())
    }
}
