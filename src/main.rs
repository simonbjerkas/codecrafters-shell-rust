use std::{
    env::{split_paths, var_os},
    fs,
    io::{self, Write},
    os::unix::fs::MetadataExt,
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
struct CommandParsingError;

enum Commands {
    Exit { status: i32 },
    Echo { display_string: String },
    Type { cmd_to_evaluate: String },
    Empty,
    Unknown { cmd: String },
}

impl Commands {
    const EXIT_CMD: &'static str = "exit";
    const ECHO_CMD: &'static str = "echo";
    const TYPE_CMD: &'static str = "type";

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
            Commands::Unknown { cmd } => {
                let key = "PATH";
                let paths = var_os(key).unwrap_or_default();

                for path in split_paths(&paths) {
                    if path.as_os_str().to_str().unwrap() == cmd {
                        if Self::is_executable(&path) {
                            return format!("{} is {}", cmd, path.display());
                        }
                    }
                }
                format!("{}: not found", cmd)
            }
        }
    }
}

impl FromStr for Commands {
    type Err = CommandParsingError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (command_type, args) = input.trim().split_once(' ').unwrap_or((input.trim(), ""));

        match command_type {
            Self::EXIT_CMD => {
                let status = args.split_whitespace().next();
                match status {
                    Some(code) => {
                        let status = code.parse();
                        match status {
                            Ok(status) => Ok(Commands::Exit { status }),
                            Err(_) => Err(CommandParsingError),
                        }
                    }
                    None => Err(CommandParsingError),
                }
            }
            Self::ECHO_CMD => Ok(Self::Echo {
                display_string: args.to_string(),
            }),
            Self::TYPE_CMD => Ok(Self::Type {
                cmd_to_evaluate: args.to_string(),
            }),
            command => Ok(Commands::Unknown {
                cmd: command.to_string(),
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
            Commands::Exit { status } => std::process::exit(status),
            Commands::Echo { display_string } => println!("{}", display_string),
            Commands::Type { cmd_to_evaluate } => {
                let evaluated_cmd = cmd_to_evaluate
                    .parse::<Commands>()
                    .unwrap_or(Commands::Empty);
                println!("{}", evaluated_cmd.type_description())
            }
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
