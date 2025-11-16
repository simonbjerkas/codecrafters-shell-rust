use std::io::{self, Write};

enum Commands {
    Exit,
    Echo { args: Vec<String> },
}

impl Commands {
    fn from_input(input: &str) -> Option<Commands> {
        let (command_type, args) = input.trim().split_once(' ').unwrap_or(("", ""));
        let command_type = command_type.to_lowercase();

        match command_type.as_str() {
            "exit" => Some(Commands::Exit),
            "echo" => Some(Commands::Echo {
                args: vec![args.to_string()],
            }),
            _ => None,
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = Commands::from_input(&input);
        match command {
            Some(Commands::Exit) => break,
            Some(Commands::Echo { args }) => println!("{}", args.join(" ")),
            None => println!("{}: command not found", input.trim()),
        }
    }
}
