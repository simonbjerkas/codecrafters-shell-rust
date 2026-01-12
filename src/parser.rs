use anyhow::Result;

use super::lexer::{Token, TokenType};
use codecrafters_shell::{Cmds, CommandStage, ParsedLine, Redirection, ShellError, redirection};

pub fn parse(tokens: Vec<Token>) -> Result<ParsedLine> {
    let pipes: Vec<Vec<Token>> = tokens
        .split(|token| token.token_type == TokenType::Pipe)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut lines = Vec::new();
    for pipe in pipes {
        let parsed = parse_command(pipe)?;
        if parsed.is_some() {
            lines.push(parsed.unwrap());
        }
    }

    Ok(ParsedLine::Pipeline(lines))
}

fn parse_command(tokens: Vec<Token>) -> Result<Option<CommandStage>> {
    let mut tokens = tokens.iter();
    let Some(cmd) = tokens.next().map(|token| {
        return Cmds::new(&token.origin);
    }) else {
        return Ok(None);
    };

    let mut args: Vec<String> = Vec::new();
    let mut redirects: Vec<Redirection> = Vec::new();
    let mut current_arg = String::new();

    while let Some(token) = tokens.next() {
        if token.token_type == TokenType::Redirects {
            let Some(path_token) = tokens.next() else {
                return Err(ShellError::MissingArg.into());
            };
            redirects.push(Redirection::new(
                redirection::eval_redirect(token.origin),
                path_token.origin,
            ));
            continue;
        }

        let origin = match token.token_type {
            TokenType::DoubleQuote => &process_escaped(token.origin),
            _ => token.origin,
        };
        current_arg.push_str(origin);

        if !token.is_adjacent {
            args.push(current_arg);
            current_arg = String::new();
        }
    }

    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    let parsed = Some(CommandStage {
        cmd,
        args,
        redirects,
    });

    Ok(parsed)
}

fn process_escaped(input: &str) -> String {
    let mut chars = input.chars();
    let mut result = String::new();

    while let Some(c) = chars.next() {
        if c != '\\' {
            result.push(c);
            continue;
        }

        match chars.next() {
            Some(next) if matches!(next, '\\' | '"' | '$' | '`' | '\n') => {
                result.push(next);
            }
            Some(next) => {
                result.push(c);
                result.push(next);
            }
            None => result.push(c),
        }
    }

    result
}
