use anyhow::Result;
use codecrafters_shell::ShellError;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Word,
    Redirects,
    Escaped,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub origin: &'a str,
    pub token_type: TokenType,
    pub is_adjacent: bool,
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
                SingleQuote,
                Escape,
                Redirection,
            }

            let compare_next = |c| match chars.clone().peekable().peek() {
                Some(next_val) => c == *next_val,
                None => false,
            };

            let started = match current {
                '"' => Started::DoubleQuote,
                '\'' => Started::SingleQuote,
                '\\' => Started::Escape,
                '>' => Started::Redirection,
                '1' if compare_next('>') => Started::Redirection,
                '2' if compare_next('>') => Started::Redirection,
                c if c.is_whitespace() => continue,
                _ => {
                    let end_index = current_str
                        .find(|c| matches!(c, '\'' | '"' | ' ' | '\n' | '\\'))
                        .unwrap_or(current_str.len());

                    let origin = &current_str[..end_index];
                    self.rest = &current_str[end_index..];

                    let is_adjacent = match self.rest.chars().peekable().peek() {
                        Some(c) if !c.is_whitespace() => true,
                        _ => false,
                    };

                    return Some(Ok(Token {
                        origin,
                        token_type: TokenType::Word,
                        is_adjacent,
                    }));
                }
            };

            match started {
                Started::DoubleQuote => {
                    let Some(end) = self.rest.find('"') else {
                        return Some(Err(ShellError::MissingQuote.into()));
                    };
                    let origin = &self.rest[..end];
                    self.rest = &self.rest[end + 1..];

                    let is_adjacent = match self.rest.chars().peekable().peek() {
                        Some(c) if !c.is_whitespace() => true,
                        _ => false,
                    };

                    return Some(Ok(Token {
                        origin,
                        token_type: TokenType::Word,
                        is_adjacent,
                    }));
                }
                Started::SingleQuote => {
                    let Some(end) = self.rest.find('\'') else {
                        return Some(Err(ShellError::MissingQuote.into()));
                    };
                    let origin = &self.rest[..end];
                    self.rest = &self.rest[end + 1..];

                    let is_adjacent = match self.rest.chars().peekable().peek() {
                        Some(c) if !c.is_whitespace() => true,
                        _ => false,
                    };

                    return Some(Ok(Token {
                        origin,
                        token_type: TokenType::Word,
                        is_adjacent,
                    }));
                }
                Started::Escape => {
                    let escaped = &self.rest[..1];
                    self.rest = &self.rest[1..];

                    let is_adjacent = match self.rest.chars().peekable().peek() {
                        Some(c) if !c.is_whitespace() => true,
                        _ => false,
                    };

                    return Some(Ok(Token {
                        origin: escaped,
                        token_type: TokenType::Escaped,
                        is_adjacent,
                    }));
                }
                Started::Redirection => {
                    let Some(next_whitespace) = current_str.find(char::is_whitespace) else {
                        return Some(Err(ShellError::MissingArg.into()));
                    };
                    let origin = current_str[..next_whitespace + 1].trim();
                    if !matches!(origin, "1>" | "1>>" | ">" | ">>" | "2>" | "2>>") {
                        return Some(Err(ShellError::Parsing.into()));
                    }
                    self.rest = &self.rest[next_whitespace + 1..];

                    let is_adjacent = match self.rest.chars().peekable().peek() {
                        Some(c) if !c.is_whitespace() => true,
                        _ => false,
                    };

                    return Some(Ok(Token {
                        origin,
                        token_type: TokenType::Redirects,
                        is_adjacent,
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
