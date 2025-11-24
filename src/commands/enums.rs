use std::{
    env::{split_paths, var_os},
    path::Path,
};

use crate::commands::{
    builtins::{unknown, Cd, Describe, Echo, Exit, Pwd},
    ShellCommand, ShellError,
};

pub enum Commands {
    Exit(Exit),
    Echo(Echo),
    Type(Describe),
    Cd(Cd),
    Pwd(Pwd),
    Unknown { cmd: String },
}

impl Commands {
    pub fn from_cmd(cmd: &str) -> Option<Self> {
        match cmd {
            "exit" => Some(Commands::Exit(Exit)),
            "echo" => Some(Commands::Echo(Echo)),
            "type" => Some(Commands::Type(Describe)),
            "pwd" => Some(Commands::Pwd(Pwd)),
            "cd" => Some(Commands::Cd(Cd)),
            command => Some(Commands::Unknown {
                cmd: command.to_string(),
            }),
        }
    }

    pub fn type_description(&self) -> String {
        match self {
            Commands::Exit(cmd) => cmd.description(),
            Commands::Echo(cmd) => cmd.description(),
            Commands::Type(cmd) => cmd.description(),
            Commands::Pwd(cmd) => cmd.description(),
            Commands::Cd(cmd) => cmd.description(),
            Commands::Unknown { cmd, .. } => {
                let key = "PATH";
                if let Some(paths) = var_os(key) {
                    for path in split_paths(&paths) {
                        let cmd_path = path.join(cmd);
                        if Path::new(&cmd_path).exists() & unknown::is_executable(&cmd_path) {
                            return format!("{} is {}", cmd, cmd_path.display());
                        }
                    }
                }

                format!("{}: not found", cmd)
            }
        }
    }

    pub fn run(&self, args: &[String]) -> Result<(), ShellError> {
        match self {
            Self::Exit(cmd) => cmd.run(args),
            Self::Echo(cmd) => cmd.run(args),
            Self::Type(cmd) => cmd.run(args),
            Self::Cd(cmd) => cmd.run(args),
            Self::Pwd(cmd) => cmd.run(args),
            Self::Unknown { cmd } => unknown::run(cmd, args),
        }
    }
}
