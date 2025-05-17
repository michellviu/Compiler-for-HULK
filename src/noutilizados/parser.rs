use crate::ast::{Expr, Operator, Program};
use crate::error::ParserError;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParserError> {
        if *self.current_token() == expected {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(
                format!("Expected {:?}, found {:?}", expected, self.current_token()),
                self.pos,
            ))
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, ParserError> {
        match self.current_token() {
            Token::Number(n) => {
                let num = *n;
                self.advance();
                Ok(Expr::Number(num))
            }
            Token::Identifier(id) => {
                let var = id.clone();
                self.advance();
                Ok(Expr::Variable(var))
            }
            Token::Keyword(k) if k == "print" => {
                self.advance();
                self.expect(Token::Symbol('('))?;
                let expr = self.parse_expr()?;
                self.expect(Token::Symbol(')'))?;
                Ok(Expr::Print(Box::new(expr)))
            }
            Token::Keyword(k) if k == "let" => {
                self.advance();
                let var = match self.current_token() {
                    Token::Identifier(id) => id.clone(),
                    _ => return Err(ParserError::ExpectedIdentifier("let binding".to_string(), self.pos)),
                };
                self.advance();
                self.expect(Token::Symbol('='))?;
                let value = self.parse_expr()?;
                self.expect(Token::Keyword("in".to_string()))?;
                let body = self.parse_expr()?;
                Ok(Expr::LetIn {
                    var,
                    value: Box::new(value),
                    body: Box::new(body),
                })
            }
            Token::Symbol('(') => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::Symbol(')'))?;
                Ok(expr)
            }
            _ => Err(ParserError::UnexpectedToken(
                format!("Unexpected token: {:?}", self.current_token()),
                self.pos,
            )),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        self.parse_binary(0)
    }

    fn parse_binary(&mut self, precedence: u8) -> Result<Expr, ParserError> {
        let mut left = self.parse_primary()?;
        loop {
            let op = match self.current_token() {
                Token::Operator(op) => match op.as_str() {
                    "+" => Some((Operator::Add, 10)),
                    "-" => Some((Operator::Subtract, 10)),
                    "*" => Some((Operator::Multiply, 20)),
                    "/" => Some((Operator::Divide, 20)),
                    _ => None,
                },
                _ => None,
            };
            if let Some((operator, new_prec)) = op {
                if new_prec < precedence {
                    break;
                }
                self.advance();
                let right = self.parse_binary(new_prec + 1)?;
                left = Expr::BinaryOp {
                    left: Box::new(left),
                    op: operator,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
    let mut exprs = Vec::new();
    // Solo procesar tokens hasta llegar a Eof
    while *self.current_token() != Token::Eof {
        let expr = self.parse_expr()?;
        exprs.push(expr);
        // Consumir punto y coma si existe
        if let Token::Symbol(';') = self.current_token() {
            self.advance();
        }
    }
    Ok(Program { expressions: exprs })
}
}
