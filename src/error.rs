use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedCharacter(char),
    UnexpectedToken { expected: String, found: String },
    ExpectedIdentifier,
    InvalidExpression,
    PrematureEOF,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedCharacter(c) => write!(f, "unexpected character '{}'", c),
            ParseError::UnexpectedToken { expected, found } => {
                write!(f, "expected '{}', found '{}'", expected, found)
            }
            ParseError::ExpectedIdentifier => write!(f, "expected identifier"),
            ParseError::InvalidExpression => write!(f, "invalid expression"),
            ParseError::PrematureEOF => write!(f, "unexpected end of input"),
        }
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
