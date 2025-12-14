use crate::error::ShellError;

enum TokenType {
    Word,
    Operator,
    Redirects,
}

pub struct Token<'a> {
    origin: &'a str,
    token_type: TokenType,
}

pub struct Lexer<'a> {
    whole: &'a str,
    rest: &'a str,
    byte: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            whole: input,
            rest: input,
            byte: 0,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, ShellError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {

            let mut chars = self.rest.chars();
            let Some(current) = chars.next() else {
                return Some(Err(ShellError::EOL));
            };

            self.rest = chars.as_str();
            self.byte += current.len_utf8();

            enum Started {
                DoubleQuote,
                SingleQuoute,
                Escape,
                Redirection
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
                    let next_whitespace = self.rest.find(char::is_whitespace).unwrap_or(self.rest.len());
                    return Some(Ok(Token { origin: &self.rest[..next_whitespace + 1], token_type: TokenType::Word }));
                }
            }

            match started {
                Started::DoubleQuote => {
                    let Some(end) = self.rest.find('"') else {
                      return Some(Err(ShellError::MissingQuote));
                    };
                    return Some(Ok(Token { origin: &self.rest[..end + 1], token_type: TokenType::Word }));
                }
                Started::SingleQuoute => {
                    let Some(end) = self.rest.find('\'') else {
                      return Some(Err(ShellError::MissingQuote));
                    };
                    return Some(Ok(Token { origin: &self.rest[..end + 1], token_type: TokenType::Word }));
                }
                Started::Escape => {
                    let skip = chars.next().unwrap_or_default();
                    self.rest = chars.as_str();
                    self.byte += skip.len_utf8();
                    continue;
                }
                Started::Redirection => {
                    return Some(Ok(Token { origin: "".to_string().as_str(), token_type: TokenType::Redirects }))
                }
            }
        }
    }
}
