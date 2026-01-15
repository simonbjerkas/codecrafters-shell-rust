mod lexer;
mod parser;
mod shell;

use anyhow::Result;
use shell::Shell;
use std::io::{self, Write};
use termion::raw::{IntoRawMode, RawTerminal};

fn main() {
    let prompt = "$ ";

    let mut stdin = io::stdin().lock();
    let mut out = Out::Raw(io::stdout().into_raw_mode().unwrap());

    let Ok(mut shell) = Shell::build() else {
        eprintln!("Failed to initialize shell");
        return;
    };

    loop {
        shell.redraw(&mut out, prompt);

        let input = shell
            .run(&mut stdin, &mut out, prompt)
            .expect("Failed to load messages from file");

        let Some(tokens) = handle_result(lexer::run_lexer(&input), &mut out, prompt, &shell) else {
            continue;
        };

        let Some(parsed) = handle_result(parser::parse(tokens), &mut out, prompt, &shell) else {
            continue;
        };

        let result = with_cooked_terminal(&mut out, || {
            codecrafters_shell::execute_pipeline(parsed, &mut shell.ctx)
        });

        match result {
            Ok(Some(res)) => {
                print_and_redraw(&mut out, prompt, &shell, &res.trim());
            }
            Ok(None) => {}
            Err(e) => print_and_redraw(&mut out, prompt, &shell, &e.to_string()),
        }
    }
}

fn print_and_redraw(out: &mut Out, prompt: &str, shell: &Shell, msg: &str) {
    write!(out, "\r{msg}\r\n").unwrap();
    out.flush().unwrap();
    shell.redraw(out, prompt);
}

enum Out {
    Cooked(io::Stdout),
    Raw(RawTerminal<io::Stdout>),
}

impl Write for Out {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Out::Cooked(o) => o.write(buf),
            Out::Raw(o) => o.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Out::Cooked(o) => o.flush(),
            Out::Raw(o) => o.flush(),
        }
    }
}

fn handle_result<T>(data: Result<T>, err: &mut Out, prompt: &str, shell: &Shell) -> Option<T> {
    match data {
        Ok(data) => return Some(data),
        Err(e) => {
            print_and_redraw(err, prompt, shell, &e.to_string());
            return None;
        }
    }
}

fn with_cooked_terminal<T>(out: &mut Out, f: impl FnOnce() -> T) -> T {
    out.flush().unwrap();

    write!(out, "\r").unwrap();
    out.flush().unwrap();

    *out = Out::Cooked(io::stdout());

    let res = f();
    out.flush().unwrap();

    *out = Out::Raw(io::stdout().into_raw_mode().unwrap());

    res
}
