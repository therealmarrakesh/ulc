use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.next_token() {
            Token::EOF => None,
            token => Some(token),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            chars: source.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.chars.peek() {
            Some(&'λ') | Some(&'\\') => {
                self.chars.next();
                Token::Lambda
            }
            Some(&'.') => {
                self.chars.next();
                Token::Dot
            }
            Some(&'(') => {
                self.chars.next();
                Token::LParen
            }
            Some(&')') => {
                self.chars.next();
                Token::RParen
            }
            Some(c) if c.is_alphabetic() => self.read_ident(),
            Some(_) => panic!("unexpected character"),
            None => Token::EOF,
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

    // Alternative: avoid String allocation by tracking byte offsets into the
    // source and slicing later (as rustc's lexer does).
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
