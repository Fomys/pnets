use std::fmt::{Display, Formatter};
use std::{error, fmt};

use pnets::arc;

use crate::parser::Position;
use crate::token;

/// All errors returned by parser
#[derive(Debug)]
pub enum ParserError {
    /// Invalid utf8 character found
    Utf8Error(Position),
    /// Invalid character at this place (example: `-e` return this error because a `-` must be followed by `>`)
    InvalidChar(Position, String),
    /// Unexpected token at this place, for example an interval at the beginning of a line
    UnexpectedToken(token::Token, String),
    /// Unexpected identifier
    UnexpectedIdentifier(Position, String),
    /// Unexpected arc
    UnexpectedArc(Position, token::Kind),
    /// Unsupported arc
    UnsupportedArc(Position, arc::Kind),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Utf8Error(pos) => write!(
                f,
                "Invalid UTF8 char at line {} column {}",
                pos.line, pos.column
            ),
            ParserError::InvalidChar(position, msg) => write!(
                f,
                "Invalid char found at line {} column {}: {}",
                position.line, position.column, msg
            ),
            ParserError::UnexpectedToken(token, msg) => write!(
                f,
                "Invalid token {} found at line {} column {}: {}",
                token.kind, token.position.line, token.position.column, msg
            ),
            ParserError::UnexpectedIdentifier(position, msg) => write!(
                f,
                "Unexpected identifier {} at line {} column {}",
                msg, position.line, position.column
            ),
            ParserError::UnexpectedArc(position, kind) => write!(
                f,
                "Unexpected arc {} at line {} column {}",
                kind, position.line, position.column
            ),
            ParserError::UnsupportedArc(position, kind) => write!(
                f,
                "Unsupported arc {} at line {} column {}",
                kind, position.line, position.column
            ),
        }
    }
}

impl error::Error for ParserError {}
