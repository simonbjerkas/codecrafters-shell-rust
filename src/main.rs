mod commands;

use commands::enums::Commands;

use std::{
    io::{self, Write},
    str::FromStr,
};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = Commands::from_str(&input).unwrap();
        match command {
            Commands::Exit { arg } => Commands::Exit { arg }.execute(),
            Commands::Echo { display_string } => Commands::Echo { display_string }.execute(),
            Commands::Type { cmd_to_evaluate } => Commands::Type { cmd_to_evaluate }.execute(),
            Commands::Unknown { cmd, args } => Commands::Unknown { cmd, args }.execute(),
            Commands::Pwd => Commands::Pwd.execute(),
            Commands::Cd { directory } => Commands::Cd { directory }.execute(),
        }
    }
}
