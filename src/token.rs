use std::fmt;

#[derive(Debug)]
pub enum Token {
    Lambda,
    Dot,
    LParen,
    RParen,
    Ident(String),
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Lambda => write!(f, "λ"),
            Token::Dot => write!(f, "."),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Ident(name) => write!(f, "{}", name),
            Token::EOF => write!(f, "end of input"),
        }
    }
}
