use crate::config::{
    ast::{Action, Rule, RuleOptions},
    lexer::{Lexer, Token},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let first = lexer.next_token().unwrap_or(Token::EOF);
        Self {
            lexer: lexer,
            current_token: first,
        }
    }

    fn advance(&mut self) -> Result<(), String> {
        self.current_token = self.lexer.next_token()?;
        Ok(())
    }

    pub fn parse_rules(&mut self) -> Result<Vec<Rule>, String> {
        let mut rules = Vec::new();
        while self.current_token != Token::EOF {
            rules.push(self.parse_rule()?);
        }
        Ok(rules)
    }

    fn parse_rule(&mut self) -> Result<Rule, String> {
        let action = match &self.current_token {
            Token::Permit => Action::Permit,
            Token::Deny => Action::Deny,
            _ => {
                return Err(format!(
                    "Line {}: Expected 'permit' or 'deny'",
                    self.lexer.line
                ));
            }
        };
        self.advance()?;

        let mut option = RuleOptions::default();
        loop {
            match &self.current_token {
                Token::Nopass => option.nopass = true,
                Token::Nolog => option.nolog = true,
                Token::Persist => option.persist = true,
                Token::KeepEnv => option.keepenv = true,
                Token::SetEnv => {
                    self.advance()?;
                    if self.current_token == Token::LBrace {
                        while self.current_token != Token::RBrace
                            && self.current_token != Token::EOF
                        {
                            self.advance()?;
                        }
                    }
                }
                _ => break,
            }
            self.advance()?;
        }

        let identity = match &self.current_token {
            Token::Text(s) => s.clone(),
            _ => {
                return Err(format!(
                    "line {}: Expected username or :group",
                    self.lexer.line
                ));
            }
        };
        self.advance()?;

        let mut target = "root".to_string();
        if self.current_token == Token::As {
            self.advance()?;
            if let Token::Text(t) = &self.current_token {
                target = t.clone();
                self.advance()?;
            } else {
                return Err(format!(
                    "line {}: Expected target user after 'as'",
                    self.lexer.line
                ));
            }
        }

        let mut cmd = None;
        if self.current_token == Token::Cmd {
            self.advance()?;
            if let Token::Text(c) = &self.current_token {
                cmd = Some(c.clone());
                self.advance()?;
            } else {
                return Err(format!(
                    "line {}: Expected command path after 'cmd'",
                    self.lexer.line
                ));
            }
        }

        let mut args = None;
        if self.current_token == Token::Args {
            self.advance()?;
            let mut args_list = Vec::new();
            while let Token::Text(a) = &self.current_token {
                args_list.push(a.clone());
                self.advance()?;
            }
            if !args_list.is_empty() {
                args = Some(args_list);
            }
        }

        Ok(Rule {
            action: action,
            options: option,
            identity: identity,
            target: target,
            cmd: cmd,
            args: args,
        })
    }
}

pub fn parse_config_file(path: &str) -> Result<Vec<Rule>, String> {
    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;
    let mut parser = Parser::new(&content);
    parser.parse_rules()
}
