use crate::{
    commands::{
        builtins::{Cd, Describe, Echo, Exit, Pwd, Unknown},
        ShellCommand, ShellError,
    },
    parser::ParsedInput,
};

pub enum Commands {
    Exit(Exit),
    Echo(Echo),
    Type(Describe),
    Cd(Cd),
    Pwd(Pwd),
    Unknown(Unknown),
}

impl Commands {
    pub fn all_commands() -> [&'static str; 5] {
        return ["exit ", "echo ", "pwd ", "cd ", "type "];
    }

    pub fn from_cmd(cmd: &str) -> Self {
        match cmd {
            "exit" => Commands::Exit(Exit),
            "echo" => Commands::Echo(Echo),
            "pwd" => Commands::Pwd(Pwd),
            "type" => Commands::Type(Describe),
            "cd" => Commands::Cd(Cd),
            command => Commands::Unknown(Unknown::new(command.to_string())),
        }
    }

    pub fn type_description(&self) -> String {
        match self {
            Commands::Exit(cmd) => cmd.description(),
            Commands::Echo(cmd) => cmd.description(),
            Commands::Type(cmd) => cmd.description(),
            Commands::Pwd(cmd) => cmd.description(),
            Commands::Cd(cmd) => cmd.description(),
            Commands::Unknown(cmd) => cmd.description(),
        }
    }

    pub fn run(&self, input: &ParsedInput) -> Result<Option<String>, ShellError> {
        match self {
            Self::Exit(cmd) => cmd.run(input),
            Self::Echo(cmd) => cmd.run(input),
            Self::Type(cmd) => cmd.run(input),
            Self::Cd(cmd) => cmd.run(input),
            Self::Pwd(cmd) => cmd.run(input),
            Self::Unknown(cmd) => cmd.run(input),
        }
    }
}
