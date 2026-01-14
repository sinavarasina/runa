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

    pub fn next_token(&mut self) {}

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
}
