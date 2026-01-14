use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Permit,
    Deny,
    Nopass,
    Nolog,
    Persist,
    KeepEnv,
    SetEnv,
    As,
    Cmd,
    Args,
    LBrace,
    RBrace,
    Text(String),
    EOF,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pub line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            line: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        match self.chars.peek() {
            None => Ok(Token::EOF),
            Some(&c) => match c {
                '#' => {
                    self.skip_comment();
                    self.next_token()
                }
                '{' => {
                    self.chars.next();
                    Ok(Token::LBrace)
                }
                '}' => {
                    self.chars.next();
                    Ok(Token::RBrace)
                }
                '"' => self.read_quoted_string(),
                _ => self.read_word(),
            },
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            match c {
                '\n' => {
                    self.line += 1;
                    self.chars.next();
                }
                c if c.is_whitespace() => {
                    self.chars.next();
                }
                _ => break,
            }
        }
    }

    fn skip_comment(&mut self) {
        while let Some(&c) = self.chars.peek() {
            match c {
                '\n' => break,
                _ => {
                    self.chars.next();
                }
            }
        }
    }

    fn read_quoted_string(&mut self) -> Result<Token, String> {
        self.chars.next();

        let mut string_content = String::new();
        while let Some(&c) = self.chars.peek() {
            match c {
                '"' => {
                    self.chars.next();
                    return Ok(Token::Text(string_content));
                }
                '\\' => {
                    self.chars.next();
                    if let Some(escaped_char) = self.chars.next() {
                        string_content.push(escaped_char);
                    }
                }
                _ => {
                    string_content.push(c);
                    self.chars.next();
                }
            }
        }
        Err(format!("Line {}: Unclosed quote string", self.line))
    }

    fn read_word(&mut self) -> Result<Token, String> {
        let mut word = String::new();
        while let Some(&c) = self.chars.peek() {
            match c {
                '#' | '{' | '}' => break,
                c if c.is_whitespace() => break,
                _ => {
                    word.push(c);
                    self.chars.next();
                }
            }
        }
        match word.as_str() {
            "permit" => Ok(Token::Permit),
            "deny" => Ok(Token::Deny),
            "nopass" => Ok(Token::Nopass),
            "nolog" => Ok(Token::Nolog),
            "persist" => Ok(Token::Persist),
            "keepenv" => Ok(Token::KeepEnv),
            "setenv" => Ok(Token::SetEnv),
            "as" => Ok(Token::As),
            "cmd" => Ok(Token::Cmd),
            "args" => Ok(Token::Args),
            _ => Ok(Token::Text(word)),
        }
    }
}
