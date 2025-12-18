use anyhow::Result;
use codecrafters_shell::ShellError;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Word,
    Redirects,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub origin: &'a str,
    pub token_type: TokenType,
}

struct Lexer<'a> {
    rest: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { rest: input }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut chars = self.rest.chars();
            let current = chars.next()?;
            let current_str = self.rest;

            self.rest = chars.as_str();

            enum Started {
                DoubleQuote,
                SingleQuoute,
                Escape,
                Redirection,
            }

            let started = match current {
                '"' => Started::DoubleQuote,
                '\'' => Started::SingleQuoute,
                '\\' => Started::Escape,
                '1' => Started::Redirection,
                '2' => Started::Redirection,
                '>' => Started::Redirection,
                c if c.is_whitespace() => continue,
                _ => {
                    let next_whitespace = current_str
                        .find(char::is_whitespace)
                        .unwrap_or(self.rest.len());

                    let origin = &current_str[..next_whitespace + 1].trim();
                    self.rest = &current_str[next_whitespace + 1..];

                    return Some(Ok(Token {
                        origin,
                        token_type: TokenType::Word,
                    }));
                }
            };

            match started {
                Started::DoubleQuote => {
                    let Some(end) = current_str.find('"') else {
                        return Some(Err(ShellError::MissingQuote.into()));
                    };
                    let origin = &current_str[..end + 1].trim();
                    self.rest = &self.rest[end..];

                    return Some(Ok(Token {
                        origin,
                        token_type: TokenType::Word,
                    }));
                }
                Started::SingleQuoute => {
                    let Some(end) = current_str.find('\'') else {
                        return Some(Err(ShellError::MissingQuote.into()));
                    };
                    let origin = &current_str[..end + 1].trim();
                    self.rest = &self.rest[end..];

                    return Some(Ok(Token {
                        origin,
                        token_type: TokenType::Word,
                    }));
                }
                Started::Escape => {
                    chars.next().unwrap_or_default();
                    self.rest = chars.as_str();
                    continue;
                }
                Started::Redirection => {
                    let Some(next_whitespace) = current_str.find(char::is_whitespace) else {
                        return Some(Err(ShellError::MissingArg.into()));
                    };
                    let origin = current_str[..next_whitespace + 1].trim();
                    if !matches!(origin, "1>" | "1>>" | ">" | ">>" | "2>" | "2>>") {
                        return Some(Err(ShellError::Parsing.into()));
                    }
                    return Some(Ok(Token {
                        origin,
                        token_type: TokenType::Redirects,
                    }));
                }
            }
        }
    }
}

pub fn run_lexer(input: &str) -> Result<Vec<Token<'_>>> {
    let lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    for token in lexer {
        let token = token?;
        tokens.push(token);
    }

    Ok(tokens)
}
