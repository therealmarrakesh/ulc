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

    pub fn parse_expression(&mut self) -> Expr {
        match self.lexer.peek() {
            // if we see a lambda, that signals to us that this is the beginning of an abstraction
            Some(Token::Lambda) => self.parse_abstraction(),

            // otherwise we dispatch to application for handling variables and parentheses
            _ => self.parse_application(),
        }
    }

    // calls parse_expression for the body
    fn parse_abstraction(&mut self) -> Expr {
        // consume the lambda
        self.lexer.next();

        // either the next token is an Ident, in which case we consume it anyway, or we call a panic, in this case we do not need to use peek()
        match self.lexer.next() {
            Some(Token::Ident(name)) => {
                match self.lexer.next() {
                    Some(Token::Dot) => {}
                    _ => panic!("expected dot"),
                }

                Expr::Abstraction {
                    parameter: name,
                    body: Rc::new(self.parse_expression()),
                }
            }
            _ => panic!("expected identifier"),
        }
    }

    // calls parse_atom repeatedly
    fn parse_application(&mut self) -> Expr {
        // we reassign result in the loop so this needs to be mut
        let mut result = self.parse_atom();

        while let Some(Token::Ident(_)) | Some(Token::LParen) | Some(Token::Lambda) =
            self.lexer.peek()
        {
            let right = match self.lexer.peek() {
                Some(Token::Lambda) => self.parse_expression(),
                _ => self.parse_atom(),
            };

            result = Expr::Application {
                left: Rc::new(result),
                right: Rc::new(right),
            };
        }

        result
    }

    // creates expression node for variables and delegates grouped expressions to parse_grouped
    fn parse_atom(&mut self) -> Expr {
        match self.lexer.next() {
            Some(Token::Ident(name)) => Expr::Variable { name },
            Some(Token::LParen) => self.parse_grouped(),
            _ => panic!("expected identifier or opening parenthesis"),
        }
    }

    // delegates to parse_expression for the inner expression and cleans up closing paren
    fn parse_grouped(&mut self) -> Expr {
        let expr = self.parse_expression();

        // consume the RParen
        match self.lexer.next() {
            Some(Token::RParen) => {}
            _ => panic!("expected closing parenthesis"),
        }

        // return the expression node returned from parse_expression
        expr
    }
}
