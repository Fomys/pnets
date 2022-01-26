//! All tokens present in a net file

use std::fmt;
use std::fmt::Formatter;

use pnets::timed::Bound;

use crate::parser::Position;

/// Token is a tokenkind linked with its position in file
#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    /// Kind of token
    pub kind: Kind,
    /// Position of token
    pub position: Position,
}

/// All tokens kind
#[derive(PartialEq, Debug, Clone)]
pub enum Kind {
    /// New line in original file
    NewLine,
    /// Net `net`
    Net,
    /// Transition `tr`
    Transition,
    /// Place `pl`
    Place,
    /// Note `nt`
    Note,
    /// Label `lb`
    Label,
    /// Priority `pr`
    Priority,
    /// Interval `('['|']')INT','INT('['|']') | ('['|']')INT','w['`
    TimeInterval(Bound, Bound),
    /// Normal arc `*`
    NormalArc,
    /// Test arc `?`
    TestArc,
    /// Inhibitor arc `?-`
    InhibitorArc,
    /// StopWatchArc `!`
    StopWatchArc,
    /// StopWatchInhibitorArc `!-`
    StopWatchInhibitorArc,
    /// Identifier ANAME or QNAME
    Identifier(String),
    /// Inline label `:`
    InlineLabel,
    /// Arrow `->`
    Arrow,
    /// Marking, weight, int
    Int(usize),
    /// Comment `#`
    Comment(String),
    /// >
    GreaterThan,
    /// <
    LessThan,
    /// End of file
    EndOfFile,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Kind::NewLine => write!(f, "NewLine"),
            Kind::Net => write!(f, "Net"),
            Kind::Transition => write!(f, "Transition"),
            Kind::Place => write!(f, "Place"),
            Kind::Note => write!(f, "Note"),
            Kind::Label => write!(f, "Label"),
            Kind::Priority => write!(f, "Priority"),
            Kind::TimeInterval(a, b) => write!(f, "TimeInterval({},{})", a, b),
            Kind::NormalArc => write!(f, "NormalArc"),
            Kind::TestArc => write!(f, "TestArc"),
            Kind::InhibitorArc => write!(f, "InhibitorArc"),
            Kind::StopWatchArc => write!(f, "StopWatchArc"),
            Kind::StopWatchInhibitorArc => write!(f, "StopWatchInhibitorArc"),
            Kind::Identifier(s) => write!(f, "Identifier({})", s),
            Kind::InlineLabel => write!(f, "InlineLabel"),
            Kind::Arrow => write!(f, "Arrow"),
            Kind::Int(v) => write!(f, "Int({})", v),
            Kind::Comment(c) => write!(f, "Comment({})", c),
            Kind::GreaterThan => write!(f, "GreaterThan"),
            Kind::LessThan => write!(f, "LessThan"),
            Kind::EndOfFile => write!(f, "EndOfFile"),
        }
    }
}
