use crate::error::LexerError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Keyword(String),
    Operator(String),
    Symbol(char),
    Eof,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    fn current_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self, n: usize) {
        for _ in 0..n {
            if let Some(ch) = self.current_char() {
                self.pos += ch.len_utf8();
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    fn lex_number(&mut self) -> Result<Token, LexerError> {
        let start = self.pos;
        let mut has_dot = false;
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.advance(1);
            } else if c == '.' && !has_dot {
                has_dot = true;
                self.advance(1);
            } else {
                break;
            }
        }
        let s = &self.input[start..self.pos];
        let num = s.parse().map_err(|_| LexerError::UnexpectedCharacter('.', start))?;
        Ok(Token::Number(num))
    }

    fn lex_identifier_or_keyword(&mut self) -> Result<Token, LexerError> {
        let start = self.pos;
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                self.advance(1);
            } else {
                break;
            }
        }
        let s = &self.input[start..self.pos];
        match s {
            "print" | "let" | "in" => Ok(Token::Keyword(s.to_string())),
            _ => Ok(Token::Identifier(s.to_string())),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        while let Some(c) = self.current_char() {
            match c {
                '0'..='9' | '.' => tokens.push(self.lex_number()?),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.lex_identifier_or_keyword()?),
                '+' | '-' | '*' | '/' => {
                    tokens.push(Token::Operator(c.to_string()));
                    self.advance(1);
                }
                ';' => {
                tokens.push(Token::Symbol(';'));
                self.advance(1);
            }
                '=' | '(' | ')' | ';' => {
                    tokens.push(Token::Symbol(c));
                    self.advance(1);
                }

                _ if c.is_whitespace() => self.skip_whitespace(),
                _ => return Err(LexerError::UnexpectedCharacter(c, self.pos)),
            }
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}
