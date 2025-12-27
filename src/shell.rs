use std::io::{self, Read, Write};

use anyhow::Result;
use codecrafters_shell::ShellError;
use termion::{event::Key, input::TermRead};

pub struct Shell {
    buffer: Vec<char>,
    cursor: usize,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            buffer: Vec::new(),
            cursor: 0,
        }
    }

    pub fn redraw<W: io::Write>(&self, out: &mut W, prompt: &str) {
        let full: String = self.buffer.iter().collect();

        write!(out, "\r{}{}{}", prompt, full, termion::clear::AfterCursor,).unwrap();

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
                // Exit
                Key::Ctrl('c') => {
                    write!(out, "\r\n").unwrap();
                    break;
                }

                // Accept line
                Key::Char('\n') | Key::Char('\r') => {
                    let line = self.current_buffer();
                    write!(out, "\r\n").unwrap();

                    // New prompt
                    self.buffer.clear();
                    self.cursor = 0;

                    return Ok(line);
                }

                // Insert printable char
                Key::Char(ch) => {
                    self.buffer.insert(self.cursor, ch);
                    self.cursor += 1;
                    self.redraw(out, prompt);
                }

                // Cursor movement
                Key::Left => {
                    self.cursor = self.cursor.saturating_sub(1);
                    self.redraw(out, prompt);
                }
                Key::Right => {
                    self.cursor = (self.cursor + 1).min(self.buffer.len());
                    self.redraw(out, prompt);
                }
                Key::Home => {
                    self.cursor = 0;
                    self.redraw(out, prompt);
                }
                Key::End => {
                    self.cursor = self.buffer.len();
                    self.redraw(out, prompt);
                }

                // Deletion
                Key::Backspace => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                        self.buffer.remove(self.cursor);
                        self.redraw(out, prompt);
                    }
                }
                Key::Delete => {
                    if self.cursor < self.buffer.len() {
                        self.buffer.remove(self.cursor);
                        self.redraw(out, prompt);
                    }
                }

                // Tab completion hook (placeholder)
                // Key::Tab => {
                //     // Insert 4 spaces as a placeholder
                //     for _ in 0..4 {
                //         buf.insert(cursor_pos, ' ');
                //         cursor_pos += 1;
                //     }
                //     redraw(&mut out, &buf, cursor_pos);
                // }
                _ => {}
            }
        }
        return Err(ShellError::Eol.into());
    }
}
