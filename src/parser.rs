use anyhow::Result;

use super::lexer::{Token, TokenType};
use codecrafters_shell::{Commands, Redirection, ShellCommand, ShellError, redirection};

pub struct ParsedInput {
    pub cmd: Box<dyn ShellCommand>,
    pub args: Vec<String>,
}

pub fn parse(tokens: Vec<Token>) -> Result<(Option<ParsedInput>, Vec<Redirection>)> {
    let mut tokens = tokens.iter();
    let Some(cmd) = tokens.next().map(|token| {
        return Commands::new(&token.origin);
    }) else {
        return Ok((None, Vec::new()));
    };

    let mut args: Vec<String> = Vec::new();
    let mut redirection: Vec<Redirection> = Vec::new();
    let mut current_arg = String::new();

    while let Some(token) = tokens.next() {
        if token.token_type == TokenType::Redirects {
            let Some(path_token) = tokens.next() else {
                return Err(ShellError::MissingArg.into());
            };
            redirection.push(Redirection::new(
                redirection::eval_redirect(token.origin),
                path_token.origin,
            ));
        }

        current_arg.push_str(token.origin);

        if !token.is_adjacent {
            args.push(current_arg);
            current_arg = String::new();
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    let parsed = Some(ParsedInput { cmd, args });

    Ok((parsed, redirection))
}
