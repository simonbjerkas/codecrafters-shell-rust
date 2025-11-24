use super::ShellError;
use std::{fs, io::Write};

pub fn parse(input: &str) -> (String, Vec<String>) {
    let mut parts = parse_command(input).unwrap_or_default();

    if parts.is_empty() {
        return (String::new(), Vec::new());
    }

    let cmd = parts.remove(0);

    (cmd, parts)
}

fn parse_command(input: &str) -> Result<Vec<String>, ShellError> {
    let mut tokens = Vec::new();
    let mut buf = String::new();

    let mut in_single = false;
    let mut in_double = false;

    let mut args = input.chars().peekable();

    let push_token = |buffer: &mut String, tokens: &mut Vec<String>| {
        if !buffer.is_empty() {
            tokens.push(std::mem::take(buffer))
        }
    };

    while let Some(c) = args.next() {
        match c {
            '\\' => {
                if in_single {
                    buf.push(c);
                    if let Some(next_char) = args.next() {
                        buf.push(next_char);
                    }
                } else if in_double {
                    match args.peek().copied() {
                        Some('$') | Some('`') | Some('"') | Some('\\') => {
                            if let Some(next_char) = args.next() {
                                buf.push(next_char);
                            }
                        }
                        Some('\n') => {
                            args.next();
                        }
                        Some(other) => {
                            buf.push(c);
                            buf.push(other);
                            args.next();
                        }
                        None => {}
                    }
                } else if let Some(next_char) = args.next() {
                    buf.push(next_char);
                }
            }
            '\'' => {
                if !in_double {
                    in_single = !in_single;
                } else {
                    buf.push(c);
                }
            }
            '"' => {
                if !in_single {
                    in_double = !in_double;
                } else {
                    buf.push(c);
                }
            }
            ch if ch.is_whitespace() && !in_single && !in_double => {
                push_token(&mut buf, &mut tokens)
            }
            _ => {
                buf.push(c);
            }
        }
    }

    if !buf.is_empty() {
        push_token(&mut buf, &mut tokens);
    }

    return Ok(tokens);
}

pub fn handle_res(output: &str, args: &[String]) -> Result<(), ShellError> {
    let mut iter = args.iter().rev().take(2);
    if let (Some(dest), Some(val)) = (iter.next(), iter.next()) {
        if val == ">" || val == "1>" {
            let file = fs::File::create(dest);

            match file {
                Ok(mut file) => {
                    if let Err(_) = file.write_all(output.as_bytes()) {
                        return Err(ShellError::WriteFile(file));
                    }
                }
                Err(_) => {
                    return Err(ShellError::CreateFile(dest.to_string()));
                }
            }
            return Ok(());
        }
    }

    println!("{}", output);

    return Ok(());
}
