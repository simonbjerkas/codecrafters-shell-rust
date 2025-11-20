use std::{
    env::{self, split_paths, var_os},
    fs,
    io::{self, Write},
    os::unix::fs::MetadataExt,
    path::Path,
    process::{self, Command},
    str::FromStr,
};

#[derive(Debug)]
struct CommandParsingError;

enum Commands {
    Exit { arg: String },
    Echo { display_string: Vec<String> },
    Type { cmd_to_evaluate: String },
    Empty,
    Unknown { cmd: String, args: Vec<String> },
    Pwd,
    Cd { directory: String },
}

fn parse_args(
    args: &str,
    expected_amount: Option<usize>,
) -> Result<Vec<String>, CommandParsingError> {
    let mut tokens = Vec::new();
    let mut buf = String::new();

    let mut in_single = false;
    let mut in_double = false;

    let mut args = args.chars().peekable();

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

    match expected_amount {
        Some(expected) => {
            if tokens.len() != expected {
                return Err(CommandParsingError);
            }
        }
        None => {}
    }

    return Ok(tokens);
}

impl Commands {
    const EXIT_CMD: &'static str = "exit";
    const ECHO_CMD: &'static str = "echo";
    const TYPE_CMD: &'static str = "type";
    const PWD_CMD: &'static str = "pwd";
    const CD_CMD: &'static str = "cd";

    fn is_executable(path: &Path) -> bool {
        fs::metadata(path)
            .map(|metadata| {
                let mode = metadata.mode();
                let owner_executable = (mode & 0o100) != 0;
                let group_executable = (mode & 0o010) != 0;
                let others_executable = (mode & 0o001) != 0;
                if owner_executable || group_executable || others_executable {
                    true
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }

    fn type_description(&self) -> String {
        match self {
            Commands::Empty => String::new(),
            Commands::Exit { .. } => format!("{} is a shell builtin", Self::EXIT_CMD),
            Commands::Echo { .. } => format!("{} is a shell builtin", Self::ECHO_CMD),
            Commands::Type { .. } => format!("{} is a shell builtin", Self::TYPE_CMD),
            Commands::Pwd => format!("{} is a shell builtin", Self::PWD_CMD),
            Commands::Cd { .. } => format!("{} is a shell builtin", Self::CD_CMD),
            Commands::Unknown { cmd, .. } => {
                let key = "PATH";
                if let Some(paths) = var_os(key) {
                    for path in split_paths(&paths) {
                        let cmd_path = path.join(cmd);
                        if Path::new(&cmd_path).exists() & Self::is_executable(&cmd_path) {
                            return format!("{} is {}", cmd, cmd_path.display());
                        }
                    }
                }

                format!("{}: not found", cmd)
            }
        }
    }

    fn execute(&self) {
        match self {
            Self::Exit { arg } => {
                let code = arg
                    .split_whitespace()
                    .next()
                    .expect("Expected code as argument");

                let status = code
                    .parse::<i32>()
                    .expect("Expected argument to be integer");
                process::exit(status)
            }
            Self::Echo { display_string } => {
                for line in display_string {
                    print!("{} ", line);
                }
                println!("")
            }
            Self::Type { cmd_to_evaluate } => {
                let evaluated_cmd = cmd_to_evaluate
                    .parse::<Commands>()
                    .unwrap_or(Commands::Empty);
                println!("{}", evaluated_cmd.type_description())
            }
            Self::Unknown { cmd, args } => {
                let key = "PATH";
                if let Some(paths) = var_os(key) {
                    for path in split_paths(&paths) {
                        let cmd_path = path.join(&cmd);
                        if Path::new(&cmd_path).exists() & Self::is_executable(&cmd_path) {
                            let mut program = Command::new(&cmd);

                            let mut process = program
                                .args(args)
                                .spawn()
                                .expect("Failed to execute command");

                            let _status = process.wait().expect("Something went wrong");
                            return;
                        }
                    }
                }
                println!("{}: command not found", cmd)
            }
            Self::Empty => {}
            Self::Pwd => {
                if let Ok(current_dir) = env::current_dir() {
                    println!("{}", current_dir.display())
                }
            }
            Self::Cd { directory } => {
                if directory.starts_with('~') {
                    let key = "HOME";
                    let home = var_os(key).unwrap_or_default();
                    let path = directory.replace('~', home.to_str().unwrap_or_default());
                    env::set_current_dir(path).unwrap();
                } else {
                    env::set_current_dir(directory).unwrap_or_else(|_| {
                        println!("cd: {}: No such file or directory", directory)
                    })
                }
            }
        }
    }
}

impl FromStr for Commands {
    type Err = CommandParsingError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (command_type, args) = input.trim().split_once(' ').unwrap_or((input.trim(), ""));

        match command_type {
            Self::EXIT_CMD => Ok(Self::Exit {
                arg: parse_args(args, Some(1))
                    .unwrap_or(vec!["".to_string()])
                    .first()
                    .unwrap()
                    .clone(),
            }),
            Self::ECHO_CMD => Ok(Self::Echo {
                display_string: parse_args(args, None).unwrap(),
            }),
            Self::TYPE_CMD => Ok(Self::Type {
                cmd_to_evaluate: args.to_string(),
            }),
            Self::PWD_CMD => Ok(Self::Pwd),
            Self::CD_CMD => Ok(Self::Cd {
                directory: args.to_string(),
            }),
            command => Ok(Commands::Unknown {
                cmd: command.to_string(),
                args: parse_args(args, None).unwrap(),
            }),
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = Commands::from_str(&input).unwrap();
        match command {
            Commands::Empty => {}
            Commands::Exit { arg } => Commands::Exit { arg }.execute(),
            Commands::Echo { display_string } => Commands::Echo { display_string }.execute(),
            Commands::Type { cmd_to_evaluate } => Commands::Type { cmd_to_evaluate }.execute(),
            Commands::Unknown { cmd, args } => Commands::Unknown { cmd, args }.execute(),
            Commands::Pwd => Commands::Pwd.execute(),
            Commands::Cd { directory } => Commands::Cd { directory }.execute(),
        }
    }
}
