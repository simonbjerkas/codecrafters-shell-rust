use std::{
    env, fs,
    io::{self, BufRead, Write},
};

use anyhow::Result;

#[derive(Clone)]
struct HistCtx {
    entries: Vec<String>,
    breakpoint: usize,
    read_path: Option<String>,
    write_path: Option<String>,
    append: bool,
}

impl HistCtx {
    pub fn build() -> Result<Self> {
        let path = env::var("HISTFILE").ok();
        let (entries, breakpoint) = match &path {
            Some(path) => {
                let file = fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .read(true)
                    .open(path)?;

                let reader = io::BufReader::new(file);
                let hist: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
                let breakpoint = hist.len();

                (hist, breakpoint)
            }
            None => (Vec::new(), 0),
        };

        Ok(HistCtx {
            entries,
            breakpoint,
            read_path: path.clone(),
            write_path: path,
            append: true,
        })
    }

    fn set_read(&mut self, path: Option<String>) -> Result<()> {
        self.read_path = path;

        let Some(path) = &self.read_path else {
            self.breakpoint = 0;
            return Ok(());
        };

        let line = format!("history -r {}", path);

        let tmp_path = format!("{}.tmp", path);

        let mut writer = io::BufWriter::new(fs::File::create(&tmp_path)?);
        writeln!(writer, "{line}")?;

        if let Ok(mut reader) = fs::File::open(&path).map(io::BufReader::new) {
            io::copy(&mut reader, &mut writer)?;
        }

        writer.flush()?;

        self.entries = fs::File::open(&tmp_path)
            .map(io::BufReader::new)?
            .lines()
            .collect::<Result<_, _>>()?;

        self.breakpoint = self.entries.len();

        fs::rename(tmp_path, path)?;
        Ok(())
    }

    fn set_write(&mut self, path: Option<String>) {
        self.write_path = path;
    }

    fn set_append(&mut self, append: bool) {
        self.append = append;
    }

    fn set_breakpoint(&mut self, breakpoint: usize) {
        self.breakpoint = breakpoint;
    }

    fn add_entry(&mut self, line: &str) {
        self.entries.push(line.to_string());
    }

    fn save_to_file(&self) -> Result<usize> {
        let Some(path) = &self.write_path else {
            return Ok(0);
        };

        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(self.append)
            .open(path)?;

        for entry in &self.entries[self.breakpoint..] {
            writeln!(file, "{entry}")?;
        }

        Ok(self.entries.len())
    }
}

pub struct ShellCtx {
    history: HistCtx,
    current_buf: Option<String>,
}

impl ShellCtx {
    pub fn build() -> Result<Self> {
        Ok(ShellCtx {
            history: HistCtx::build()?,
            current_buf: None,
        })
    }

    pub fn set_read_history(&mut self, path: &str) -> Result<()> {
        self.history.set_read(Some(path.to_string()))?;

        Ok(())
    }

    pub fn set_write_history(&mut self, path: &str) -> Result<()> {
        self.history.set_append(false);
        self.history.set_write(Some(path.to_string()));
        let bp = self.history.save_to_file()?;
        self.history.set_breakpoint(bp);

        Ok(())
    }

    pub fn set_append_history(&mut self, path: &str) -> Result<()> {
        self.history.set_append(true);
        self.history.set_write(Some(path.to_string()));
        let bp = self.history.save_to_file()?;
        self.history.set_breakpoint(bp);

        Ok(())
    }

    pub fn handle_history(&mut self, line: &str) {
        self.current_buf = None;
        self.history.add_entry(line);
    }

    pub fn shut_down(&self) -> Result<()> {
        self.history.save_to_file()?;

        Ok(())
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history.entries
    }

    pub fn get_history_entry(&mut self, pos: usize, current: String) -> String {
        let current_buf = self.current_buf.get_or_insert(current);

        self.history
            .entries
            .iter()
            .rev()
            .nth(pos.saturating_sub(1))
            .cloned()
            .unwrap_or_else(|| current_buf.clone())
    }
}
