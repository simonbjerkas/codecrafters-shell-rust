use super::{Commands, ShellError};

pub struct ParsedInput {
    pub cmd: Commands,
    pub args: Vec<String>,
    pub output: OutputStyle,
}

pub enum OutputStyle {
    Print,
    StdOut { path: String, append: bool },
    StdErr { path: String, append: bool },
}

pub fn parse(input: &str) -> Option<ParsedInput> {
    let mut parts = parse_command(input).unwrap_or_default();

    if parts.is_empty() {
        return None;
    }

    let first_input = parts.remove(0);

    let cmd = Commands::from_cmd(&first_input);

    for (idx, arg) in parts.iter().enumerate() {
        if arg == ">" || arg == "1>" {
            parts.remove(idx);
            let path = parts.remove(idx);
            return Some(ParsedInput {
                cmd,
                args: parts,
                output: OutputStyle::StdOut {
                    path,
                    append: false,
                },
            });
        } else if arg == "2>" {
            parts.remove(idx);
            let path = parts.remove(idx);
            return Some(ParsedInput {
                cmd,
                args: parts,
                output: OutputStyle::StdErr {
                    path,
                    append: false,
                },
            });
        } else if arg == ">>" || arg == "1>>" {
            parts.remove(idx);
            let path = parts.remove(idx);
            return Some(ParsedInput {
                cmd,
                args: parts,
                output: OutputStyle::StdOut { path, append: true },
            });
        } else if arg == "2>>" {
            parts.remove(idx);
            let path = parts.remove(idx);
            return Some(ParsedInput {
                cmd,
                args: parts,
                output: OutputStyle::StdErr { path, append: true },
            });
        }
    }

    Some(ParsedInput {
        cmd,
        args: parts,
        output: OutputStyle::Print,
    })
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
