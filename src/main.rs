use std::io::{self, Write};

enum Commands {
    Exit { args: i32 },
    Echo { args: String },
    Type { args: String },
}

const COMMANDS: [&str; 3] = ["exit", "echo", "type"];

impl Commands {
    fn from_input(input: &str) -> Option<Commands> {
        let mut input = input.trim().split_whitespace();
        let command_type = input.next().unwrap_or("").to_lowercase();

        match command_type.as_str() {
            "exit" => {
                let args = input.next();
                match args {
                    Some(arg) => match arg.parse::<i32>() {
                        Ok(num) => Some(Commands::Exit { args: num }),
                        Err(_) => None,
                    },
                    None => panic!("Missing argument for exit command"),
                }
            }
            "echo" => Some(Commands::Echo {
                args: input.collect(),
            }),
            "type" => Some(Commands::Type {
                args: input.collect(),
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
            Some(Commands::Exit { args }) => {
                if args > 1 {
                    panic!("Ughh")
                } else {
                    std::process::exit(args);
                }
            }
            Some(Commands::Echo { args }) => println!("{}", args),
            Some(Commands::Type { args }) => {
                if COMMANDS.contains(&args.as_str()) {
                    println!("{} is a shell builtin", args);
                } else {
                    println!("{}: not found", args);
                }
            }
            None => println!("{}: command not found", input.trim()),
        }
    }
}
