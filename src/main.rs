#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();

        if io::stdin().read_line(&mut command).unwrap() == 0 {
            println!("Exiting...");
            break;
        }

        let command = command.trim();
        if command.is_empty() {
            continue;
        }

        println!("{}: command not found", command);
    }
}
