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

pub fn parse(input: &str) -> Vec<Option<ParsedInput>> {
    let cmds = parse_command(input).unwrap_or_default();

    let mut results = Vec::new();

    for mut parts in cmds {
        if parts.is_empty() {
            results.push(None);
            continue;
        }

        let command_name = parts.remove(0);
        let cmd = Commands::from_cmd(&command_name);

        let mut output = OutputStyle::Print;

        let mut redirect_idx = None;

        for (idx, arg) in parts.iter().enumerate() {
            if matches!(arg.as_str(), ">" | "1>" | "2>" | ">>" | "1>>" | "2>>") {
                redirect_idx = Some(idx);
                break;
            }
        }

        if let Some(idx) = redirect_idx {
            if idx + 1 < parts.len() {
                let operator = parts.remove(idx);
                let path = parts.remove(idx);

                output = match operator.as_str() {
                    ">" | "1>" => OutputStyle::StdOut {
                        path,
                        append: false,
                    },
                    ">>" | "1>>" => OutputStyle::StdOut { path, append: true },
                    "2>" => OutputStyle::StdErr {
                        path,
                        append: false,
                    },
                    "2>>" => OutputStyle::StdErr { path, append: true },
                    _ => OutputStyle::Print,
                }
            }
        }

        results.push(Some(ParsedInput {
            cmd,
            args: parts,
            output,
        }))
    }

    results
}

pub fn parse_command(input: &str) -> Result<Vec<Vec<String>>, ShellError> {
    let mut results = Vec::new();
    let lines = split_commands(input);

    let push_token = |buffer: &mut String, tokens: &mut Vec<String>| {
        if !buffer.is_empty() {
            tokens.push(std::mem::take(buffer))
        }
    };

    for input in lines {
        let mut tokens = Vec::new();
        let mut buf = String::new();

        let mut in_single = false;
        let mut in_double = false;

        let mut args = input.chars().peekable();

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

        results.push(tokens);
    }

    Ok(results)
}

fn split_commands(line: &str) -> Vec<String> {
    let mut cmds = Vec::new();

    let mut last_pos = 0;
    for (idx, arg) in line.chars().enumerate() {
        if arg == '|' {
            cmds.push(line[last_pos..idx].to_string());
            last_pos = idx + 1;
        }
    }

    cmds.push(line[last_pos..].to_string());

    cmds
}
