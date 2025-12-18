use anyhow::Result;

use super::lexer::{Token, TokenType};
use codecrafters_shell::{Commands, Redirection, ShellCommand, redirection};

pub struct ParsedInput {
    pub cmd: Box<dyn ShellCommand>,
    pub args: Vec<String>,
}

pub fn parse(tokens: Vec<Token>) -> Result<(Option<ParsedInput>, Option<Redirection>)> {
    let mut tokens = tokens.iter();
    let Some(cmd) = tokens.next().map(|token| {
        return Commands::new(&token.origin);
    }) else {
        return Ok((None, None));
    };

    let mut args: Vec<String> = Vec::new();
    let mut redirection = None;

    while let Some(token) = tokens.next() {
        if token.token_type == TokenType::Redirects {
            redirection = Some(Redirection::new(
                redirection::eval_redirect(token.origin),
                tokens.next().unwrap().origin,
            ));
            break;
        }

        args.push(token.origin.to_string());
    }

    let parsed = Some(ParsedInput { cmd, args });

    Ok((parsed, redirection))
}
