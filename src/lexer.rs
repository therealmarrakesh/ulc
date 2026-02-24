use crate::error::{ParseError, ParseResult};
use crate::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = ParseResult<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Token::EOF) => None,
            Ok(token) => Some(Ok(token)),
            Err(e) => Some(Err(e)),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            chars: source.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> ParseResult<Token> {
        self.skip_whitespace();

        match self.chars.peek() {
            Some(&'λ') | Some(&'\\') => {
                self.chars.next();
                Ok(Token::Lambda)
            }
            Some(&'.') => {
                self.chars.next();
                Ok(Token::Dot)
            }
            Some(&'(') => {
                self.chars.next();
                Ok(Token::LParen)
            }
            Some(&')') => {
                self.chars.next();
                Ok(Token::RParen)
            }
            Some(c) if c.is_alphabetic() => Ok(self.read_ident()),
            Some(&c) => Err(ParseError::UnexpectedCharacter(c)),
            None => Ok(Token::EOF),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_whitespace() => {
                    self.chars.next();
                }
                _ => break,
            }
        }
    }

    fn read_ident(&mut self) -> Token {
        let mut buf = String::new();
        loop {
            match self.chars.peek() {
                Some(&c) if c.is_alphanumeric() || c == '_' => {
                    buf.push(c);
                    self.chars.next();
                }
                _ => break,
            }
        }
        Token::Ident(buf)
    }
}
