use std::error::Error;
use std::io::BufRead;

use pnets::timed::Bound;

use crate::parser::Position;
use crate::reader::Reader;
use crate::token::{Kind, Token};
use crate::ParserError;

/// Lexer for net files
pub struct Lexer<R: BufRead> {
    reader: Reader<R>,
    pub current_token: Token,
    next_token: Option<Token>,
    tmp_string: String,
}

impl<R: BufRead> Lexer<R> {
    /// Create a new lexer for reading .net files
    ///
    /// ```ignore
    /// let lexer = Reader::new(&"This string will be lexed".as_bytes());
    /// ```
    pub fn new(reader: R) -> Self {
        Self {
            reader: Reader::new(reader),
            current_token: Token {
                kind: Kind::EndOfFile,
                position: Position { line: 1, column: 0 },
            },
            next_token: None,
            tmp_string: "".to_string(),
        }
    }

    /// Check if a char can be used in alphanumeric (ANAME) identifier
    fn is_ident_char(ch: char) -> bool {
        ch.is_alphanumeric() || ch == '_' || ch == '\''
    }

    /// Try to parse some text token or generate an ANAME Identifier
    fn parse_identifer(&mut self) -> Result<Kind, Box<dyn Error>> {
        self.tmp_string.clear();
        self.tmp_string.push(self.reader.read()?);
        // Bug probable pour le parser GO: "{ \} }" => Donne "{ \}" au lieu de "{ \} }"
        while Self::is_ident_char(self.reader.peek()?) {
            self.tmp_string.push(self.reader.read()?);
        }

        Ok(match self.tmp_string.as_str() {
            "TR" | "Tr" | "tR" | "tr" => Kind::Transition,
            "NET" | "NEt" | "NeT" | "nET" | "Net" | "nEt" | "neT" | "net" => Kind::Net,
            "LB" | "Lb" | "lB" | "lb" => Kind::Label,
            "NT" | "Nt" | "nT" | "nt" | "na" | "NA" | "Na" | "nA" => Kind::Note,
            "PL" | "Pl" | "pL" | "pl" => Kind::Place,
            "PR" | "Pr" | "pR" | "pr" => Kind::Priority,
            _ => Kind::Identifier(self.tmp_string.clone()),
        })
    }

    /// Parse int from reader
    fn parse_small_int(&mut self) -> Result<usize, Box<dyn Error>> {
        self.tmp_string.clear();
        while self.reader.peek()?.is_numeric() {
            self.tmp_string.push(self.reader.read()?);
        }

        Ok(self.tmp_string.parse()?)
    }

    /// Parse time interval
    fn parse_time_interval(&mut self) -> Result<Kind, Box<dyn Error>> {
        let open = match self.reader.read()? {
            '[' => Bound::Closed(self.parse_int()?),
            ']' => Bound::Open(self.parse_int()?),
            _ => {
                return Err(Box::new(ParserError::InvalidChar(
                    self.reader.current_position,
                    "[, ]".to_string(),
                )));
            }
        };

        if self.reader.read()? != ',' {
            return Err(Box::new(ParserError::InvalidChar(
                self.reader.current_position,
                ",".to_string(),
            )));
        }

        let close = if self.reader.peek()? == 'w' {
            self.reader.read()?;
            if self.reader.read()? != '[' {
                return Err(Box::new(ParserError::InvalidChar(
                    self.reader.current_position,
                    "[".to_string(),
                )));
            }
            Bound::Infinity
        } else {
            let end = self.parse_int()?;
            match self.reader.read()? {
                ']' => Bound::Closed(end),
                '[' => Bound::Open(end),
                _ => {
                    return Err(Box::new(ParserError::InvalidChar(
                        self.reader.current_position,
                        "[, ]".to_string(),
                    )));
                }
            }
        };

        Ok(Kind::TimeInterval(open, close))
    }

    /// Parse int with unit
    fn parse_int(&mut self) -> Result<usize, Box<dyn Error>> {
        let complex = if self.reader.peek()? == '(' {
            self.reader.read()?;
            true
        } else {
            false
        };
        let r = self.parse_small_int()?
            * (match self.reader.peek()? {
                'K' => {
                    self.reader.read()?;
                    1_000
                }
                'M' => {
                    self.reader.read()?;
                    1_000_000
                }
                _ => 1,
            });
        if complex & (self.reader.peek()? == ')') {
            self.reader.read()?;
        }
        Ok(r)
    }

    /// Parse comment line
    fn parse_comment(&mut self) -> Result<Kind, Box<dyn Error>> {
        self.tmp_string.clear();
        self.reader.read()?; // Remove trailing #

        while self.reader.peek()? != '\n' {
            self.tmp_string.push(self.reader.read()?);
        }

        Ok(Kind::Comment(self.tmp_string.clone()))
    }

    /// Parse next token
    fn parse_next_token(&mut self) -> Result<Token, Box<dyn Error>> {
        // Remove whitespaces
        while matches!(self.reader.peek()?, ' ' | '\t' | '\r') {
            self.reader.read()?;
        }
        Ok(Token {
            position: self.reader.next_position,
            kind: match (&self.current_token.kind, self.reader.peek()?) {
                (_, '\n') => {
                    self.reader.read()?;
                    Kind::NewLine
                }
                (_, '-') => {
                    self.reader.read()?;
                    if self.reader.read()? == '>' {
                        Kind::Arrow
                    } else {
                        return Err(Box::new(ParserError::InvalidChar(
                            self.reader.current_position,
                            "expected >".to_string(),
                        )));
                    }
                }
                (_, '[') | (_, ']') => self.parse_time_interval()?,
                (_, ':') => {
                    self.reader.read()?;
                    Kind::InlineLabel
                }
                (_, '*') => {
                    self.reader.read()?;
                    Kind::NormalArc
                }
                (_, '?') => {
                    self.reader.read()?;
                    match self.reader.peek()? {
                        '-' => {
                            self.reader.read()?;
                            Kind::InhibitorArc
                        }
                        _ => Kind::TestArc,
                    }
                }
                (_, '!') => {
                    self.reader.read()?;
                    match self.reader.peek()? {
                        '-' => Kind::StopWatchInhibitorArc,
                        _ => Kind::StopWatchArc,
                    }
                }
                (_, '#') => self.parse_comment()?,
                (_, '(') => Kind::Int(self.parse_int()?),
                (Kind::Identifier(_), c)
                | (Kind::NormalArc, c)
                | (Kind::TestArc, c)
                | (Kind::InhibitorArc, c)
                | (Kind::StopWatchArc, c)
                | (Kind::StopWatchInhibitorArc, c)
                    if c.is_numeric() =>
                {
                    Kind::Int(self.parse_int()?)
                }
                (_, '{') => self.parse_escaped_identifer()?,
                (_, '\u{0}') => Kind::EndOfFile,
                (_, '>') => {
                    self.reader.read()?;
                    Kind::GreaterThan
                }
                (_, '<') => {
                    self.reader.read()?;
                    Kind::LessThan
                }
                (_, _) => self.parse_identifer()?,
                /*_ => {
                    return Err(Box::new(ParserError::InvalidChar(
                        self.reader.current_position,
                        "This character is not valid".to_string(),
                    )));
                }*/
            },
        })
    }

    fn parse_escaped_identifer(&mut self) -> Result<Kind, Box<dyn Error>> {
        self.tmp_string.clear();
        let mut last = ' ';
        self.reader.read()?;
        while (self.reader.peek()? != '}') || ((self.reader.peek()? == '}') & (last == '\\')) {
            match (last, self.reader.peek()?) {
                ('\\', '\\') => self.tmp_string.push('\\'),
                ('\\', '{') => self.tmp_string.push('{'),
                ('\\', '}') => self.tmp_string.push('}'),
                (_, '{') | (_, '}') => {
                    return Err(Box::new(ParserError::InvalidChar(
                        self.reader.current_position,
                        "\\, { and } must be precedeed by".to_string(),
                    )));
                }
                (_, '\\') => {}
                (_, c) => self.tmp_string.push(c),
            }
            last = self.reader.read()?;
        }
        self.reader.read()?;
        Ok(Kind::Identifier(self.tmp_string.clone()))
    }

    /// Peek next token
    pub fn peek(&mut self) -> Result<Token, Box<dyn Error>> {
        if self.next_token.is_none() {
            self.next_token = Some(self.parse_next_token()?);
        }
        Ok(self.next_token.as_ref().unwrap().clone())
    }

    /// Read next token
    pub fn read(&mut self) -> Result<Token, Box<dyn Error>> {
        let t = self.peek()?;
        self.current_token = self.next_token.clone().unwrap();
        self.next_token = None;
        Ok(t)
    }
}

#[allow(unused_imports)]
mod tests {
    use pnets::timed::Bound;

    use crate::lexer::Lexer;
    use crate::parser::Position;
    use crate::token::{Kind, Token};
    use crate::ParserError;

    #[test]
    fn test_complex_identifier() {
        let mut lexer = Lexer::new("{Complex \\{\\}\\\\ identifier}".as_bytes());
        assert_eq!(
            lexer.read().unwrap(),
            Token {
                kind: Kind::Identifier("Complex {}\\ identifier".to_string()),
                position: Position { line: 1, column: 1 },
            }
        )
    }

    #[test]
    fn test_invalid_identifier() {
        let mut lexer = Lexer::new("{{}".as_bytes());
        let r = lexer.read();
        assert!(r.is_err());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_peek() {
        let mut lexer = Lexer::new("\nnet *".as_bytes());
        lexer.peek();
        assert_eq!(
            lexer.peek().unwrap(),
            Token {
                kind: Kind::NewLine,
                position: Position { line: 1, column: 1 },
            }
        );
        lexer.read();
        assert_eq!(
            lexer.peek().unwrap(),
            Token {
                kind: Kind::Net,
                position: Position { line: 2, column: 1 },
            }
        );
    }

    #[test]
    fn test_newline() {
        let mut lexer = Lexer::new("\n\n".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::NewLine,
                position: Position { line: 1, column: 1 },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::NewLine,
                position: Position { line: 2, column: 1 },
            }
        );
    }

    #[test]
    fn test_whitespace() {
        let mut lexer = Lexer::new("   \r\n\t\t\r\n".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::NewLine,
                position: Position { line: 1, column: 5 },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::NewLine,
                position: Position { line: 2, column: 4 },
            }
        );
    }

    #[test]
    fn test_tr() {
        let mut lexer = Lexer::new("tr".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Transition,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_gt() {
        let mut lexer = Lexer::new(">".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::GreaterThan,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_lt() {
        let mut lexer = Lexer::new("<".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::LessThan,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_comments() {
        let mut lexer = Lexer::new("#tr\nnet  net".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Comment("tr".to_string()),
                position: Position { line: 1, column: 1 },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::NewLine,
                position: Position { line: 1, column: 4 },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Net,
                position: Position { line: 2, column: 1 },
            }
        );
    }

    #[test]
    fn test_net() {
        let mut lexer = Lexer::new("net".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Net,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_pl() {
        let mut lexer = Lexer::new("pl".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Place,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_nt() {
        let mut lexer = Lexer::new("nt".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Note,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_lb() {
        let mut lexer = Lexer::new("lb".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Label,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_pr() {
        let mut lexer = Lexer::new("pr".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Priority,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_stopwatch() {
        let mut lexer = Lexer::new("!".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::StopWatchArc,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_stopwatchinhibitor() {
        let mut lexer = Lexer::new("!-".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::StopWatchInhibitorArc,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::new("label_comp'lex".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Identifier("label_comp'lex".to_string()),
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_time_interval() {
        let mut lexer = Lexer::new("[0,1][0,w[]0,1[".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::TimeInterval(Bound::Closed(0), Bound::Closed(1)),
                position: Position { line: 1, column: 1 },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::TimeInterval(Bound::Closed(0), Bound::Infinity),
                position: Position { line: 1, column: 6 },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::TimeInterval(Bound::Open(0), Bound::Open(1)),
                position: Position {
                    line: 1,
                    column: 11,
                },
            }
        );
    }

    #[test]
    fn test_arrow() {
        let mut lexer = Lexer::new("->".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Arrow,
                position: Position { line: 1, column: 1 },
            }
        );
    }

    #[test]
    fn test_invalid_arrow() {
        let mut lexer = Lexer::new("-a".as_bytes());
        assert!(lexer.parse_next_token().is_err());
    }

    #[test]
    fn test_full_line() {
        let mut lexer = Lexer::new("tr t0 : a [1,1] p0*3 -> p1".as_bytes());
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Transition,
                position: Position { line: 1, column: 1 },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Identifier("t0".to_string()),
                position: Position { line: 1, column: 4 },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::InlineLabel,
                position: Position { line: 1, column: 7 },
            }
        );

        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Identifier("a".to_string()),
                position: Position { line: 1, column: 9 },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::TimeInterval(Bound::Closed(1), Bound::Closed(1)),
                position: Position {
                    line: 1,
                    column: 11,
                },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Identifier("p0".to_string()),
                position: Position {
                    line: 1,
                    column: 17,
                },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::NormalArc,
                position: Position {
                    line: 1,
                    column: 19,
                },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Identifier("3".to_string()),
                position: Position {
                    line: 1,
                    column: 20,
                },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Arrow,
                position: Position {
                    line: 1,
                    column: 22,
                },
            }
        );
        assert_eq!(
            lexer.parse_next_token().unwrap(),
            Token {
                kind: Kind::Identifier("p1".to_string()),
                position: Position {
                    line: 1,
                    column: 25,
                },
            }
        );
    }

    #[test]
    fn test_invalid_time_interval() {
        let mut lexer = Lexer::new("a".as_bytes());
        assert!(lexer.parse_time_interval().is_err());
        lexer = Lexer::new("[a,w]".as_bytes());
        assert!(lexer.parse_time_interval().is_err());
        lexer = Lexer::new("[45w]".as_bytes());
        assert!(lexer.parse_time_interval().is_err());
        lexer = Lexer::new("[45,34a".as_bytes());
        assert!(lexer.parse_time_interval().is_err());
        lexer = Lexer::new("[45,a]".as_bytes());
        assert!(lexer.parse_time_interval().is_err());
        lexer = Lexer::new("[45,w]".as_bytes());
        assert!(lexer.parse_time_interval().is_err());
        lexer = Lexer::new("[45,wa".as_bytes());
        assert!(lexer.parse_time_interval().is_err());
    }
}
