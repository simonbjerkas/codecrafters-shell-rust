use std::io::{self, Read, Write};

use anyhow::Result;
use codecrafters_shell::{Builtins, ShellError};
use termion::{event::Key, input::TermRead};

pub struct Shell {
    buffer: Vec<char>,
    cursor: usize,
    last_event: Option<Key>,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            buffer: Vec::new(),
            cursor: 0,
            last_event: None,
        }
    }

    pub fn redraw<W: io::Write>(&self, out: &mut W, prompt: &str) {
        let full: String = self.buffer.iter().collect();

        let prompt_len = prompt.chars().count();
        let cursor_col = (prompt_len + self.cursor) as u16;

        write!(
            out,
            "\r{}{}{}\r{}",
            prompt,
            full,
            termion::clear::AfterCursor,
            termion::cursor::Right(cursor_col)
        )
        .unwrap();

        out.flush().unwrap();
    }

    fn current_buffer(&self) -> String {
        self.buffer.iter().collect()
    }

    pub fn run<W, R>(&mut self, stdin: R, out: &mut W, prompt: &str) -> Result<String>
    where
        W: Write,
        R: Read,
    {
        for key in stdin.keys() {
            let key = key.unwrap();

            match key {
                Key::Ctrl('c') => {
                    write!(out, "\r\n").unwrap();
                    break;
                }

                Key::Char('\n') | Key::Char('\r') => {
                    self.last_event = None;
                    let line = self.current_buffer();
                    write!(out, "\r\n").unwrap();

                    self.buffer.clear();
                    self.cursor = 0;

                    return Ok(line);
                }

                Key::Char('\t') if self.last_event == Some(Key::Char('\t')) => {
                    let partial = self.current_buffer();

                    let possibilities = get_possibilities(&partial)?;

                    write!(out, "\n\r{}", possibilities.join("  ")).unwrap();

                    writeln!(out).unwrap();
                    self.redraw(out, prompt);
                    self.last_event = None;
                }
                Key::Char('\t') => {
                    let partial = self.current_buffer();

                    let possibilities = get_possibilities(&partial)?;

                    if !possibilities.is_empty() {
                        self.last_event = Some(Key::Char('\t'));
                    }

                    let common_prefix = common_prefix_ascii(&possibilities);

                    if possibilities.len() == 1 {
                        self.buffer = possibilities.first().unwrap().trim().chars().collect();
                        self.buffer.push(' ');
                        self.cursor = self.buffer.len();
                        self.last_event = None;
                    } else if !common_prefix.is_empty() && common_prefix != self.current_buffer() {
                        self.buffer = common_prefix.trim().chars().collect();
                        self.cursor = self.buffer.len();
                    } else {
                        write!(out, "\x07").unwrap();
                    }

                    self.redraw(out, prompt);
                }

                Key::Char(ch) => {
                    self.last_event = Some(Key::Char(ch));
                    self.buffer.insert(self.cursor, ch);
                    self.cursor += 1;
                    self.redraw(out, prompt);
                }

                Key::Left => {
                    self.last_event = Some(Key::Left);
                    self.cursor = self.cursor.saturating_sub(1);
                    self.redraw(out, prompt);
                }
                Key::Right => {
                    self.last_event = Some(Key::Right);
                    self.cursor = (self.cursor + 1).min(self.buffer.len());
                    self.redraw(out, prompt);
                }
                Key::Home => {
                    self.last_event = Some(Key::Home);
                    self.cursor = 0;
                    self.redraw(out, prompt);
                }
                Key::End => {
                    self.last_event = Some(Key::End);
                    self.cursor = self.buffer.len();
                    self.redraw(out, prompt);
                }

                Key::Backspace => {
                    self.last_event = Some(Key::Backspace);
                    if self.cursor > 0 {
                        self.cursor -= 1;
                        self.buffer.remove(self.cursor);
                        self.redraw(out, prompt);
                    }
                }
                Key::Delete => {
                    self.last_event = Some(Key::Delete);
                    if self.cursor < self.buffer.len() {
                        self.buffer.remove(self.cursor);
                        self.redraw(out, prompt);
                    }
                }

                _ => {}
            }
        }
        return Err(ShellError::Eol.into());
    }
}

fn get_possibilities(partial: &str) -> Result<Vec<String>> {
    let mut possibilities: Vec<String> = Builtins::all_builtins()
        .iter()
        .filter(|cmd| cmd.starts_with(partial))
        .map(|cmd| cmd.to_string())
        .collect();

    let externals = codecrafters_shell::search_executables(partial)?;

    possibilities.extend(externals.iter().cloned());

    let mut possibilities: Vec<String> = possibilities
        .iter()
        .map(|cmd| cmd.trim().to_string())
        .collect();

    possibilities.sort();
    possibilities.dedup();

    Ok(possibilities)
}

fn common_prefix_ascii(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first = strings[0].as_bytes();
    let mut end = first.len();

    for s in &strings[1..] {
        let b = s.as_bytes();
        end = end.min(b.len());

        for i in 0..end {
            if first[i] != b[i] {
                end = i;
                break;
            }
        }
    }

    strings[0][..end].to_string()
}
