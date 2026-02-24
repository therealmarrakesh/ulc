use crate::error::{ParseError, ParseResult};
use crate::expr::Expr;
use crate::lexer::Lexer;
use crate::token::Token;
use std::iter::Peekable;
use std::rc::Rc;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse(&mut self) -> ParseResult<Expr> {
        let expr = self.parse_expression()?;

        match self.lexer.next() {
            None => Ok(expr),
            Some(Ok(t)) => Err(ParseError::UnexpectedToken {
                expected: "end of input".to_string(),
                found: format!("{}", t),
            }),
            Some(Err(e)) => Err(e),
        }
    }

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        match self.lexer.peek() {
            // if we see a lambda, that signals to us that this is the beginning of an abstraction
            Some(Ok(Token::Lambda)) => self.parse_abstraction(),
            Some(Err(_)) => Err(self.lexer.next().unwrap().unwrap_err()),

            // otherwise we dispatch to application for handling variables and parentheses
            _ => self.parse_application(),
        }
    }

    // calls parse_expression for the body
    fn parse_abstraction(&mut self) -> ParseResult<Expr> {
        // consume the lambda
        self.lexer.next();

        match self.lexer.next() {
            Some(Ok(Token::Ident(name))) => {
                match self.lexer.next() {
                    Some(Ok(Token::Dot)) => (),
                    Some(Ok(t)) => {
                        return Err(ParseError::UnexpectedToken {
                            expected: ".".to_string(),
                            found: format!("{}", t),
                        });
                    }
                    Some(Err(e)) => return Err(e),
                    None => return Err(ParseError::PrematureEOF),
                }

                Ok(Expr::Abstraction {
                    parameter: name,
                    body: Rc::new(self.parse_expression()?),
                })
            }
            Some(Ok(_)) => Err(ParseError::ExpectedIdentifier),
            Some(Err(e)) => Err(e),
            None => Err(ParseError::PrematureEOF),
        }
    }

    // calls parse_atom repeatedly
    fn parse_application(&mut self) -> ParseResult<Expr> {
        // we reassign result in the loop so this needs to be mut
        let mut result = self.parse_atom()?;

        while let Some(Ok(Token::Ident(_))) | Some(Ok(Token::LParen)) | Some(Ok(Token::Lambda)) =
            self.lexer.peek()
        {
            let right = match self.lexer.peek() {
                Some(Ok(Token::Lambda)) => self.parse_expression()?,
                _ => self.parse_atom()?,
            };

            result = Expr::Application {
                left: Rc::new(result),
                right: Rc::new(right),
            };
        }

        Ok(result)
    }

    // creates expression node for variables and delegates grouped expressions to parse_grouped
    fn parse_atom(&mut self) -> ParseResult<Expr> {
        match self.lexer.next() {
            Some(Ok(Token::Ident(name))) => Ok(Expr::Variable { name }),
            Some(Ok(Token::LParen)) => self.parse_grouped(),
            Some(Ok(t)) => Err(ParseError::UnexpectedToken {
                expected: "identifier or (".to_string(),
                found: format!("{}", t),
            }),
            Some(Err(e)) => Err(e),
            None => Err(ParseError::PrematureEOF),
        }
    }

    // delegates to parse_expression for the inner expression and cleans up closing paren
    fn parse_grouped(&mut self) -> ParseResult<Expr> {
        let expr = self.parse_expression()?;

        // consume the RParen
        match self.lexer.next() {
            Some(Ok(Token::RParen)) => Ok(expr),
            Some(Ok(t)) => Err(ParseError::UnexpectedToken {
                expected: ")".to_string(),
                found: format!("{}", t),
            }),
            Some(Err(e)) => Err(e),
            None => Err(ParseError::PrematureEOF),
        }
    }
}
