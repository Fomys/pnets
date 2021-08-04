use std::error::Error;

use pnets::timed::{Net, TimeRange};
use pnets::{arc, NetError, NodeId};
use pnets::{PlaceId, TransitionId};

use crate::lexer::Lexer;
use crate::token;
use crate::token::Kind;
use crate::ParserError;
use std::io::BufRead;

/// Position in a file
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    /// Current line
    pub line: usize,
    /// Current column
    pub column: usize,
}

/// Exporter for [tina]() format.
///
/// It consume a reader and creates a [`pnets::timed::Net`]
pub struct Parser<R: BufRead> {
    lexer: Lexer<R>,
    net: Net,
}

impl<R: BufRead> Parser<R> {
    /// Parse a timed net from a reader
    pub fn parse(mut self) -> Result<Net, Box<dyn Error>> {
        loop {
            let token = self.lexer.peek()?;

            match token.kind {
                token::Kind::NewLine | token::Kind::Comment(_) => {
                    self.lexer.read()?;
                }
                token::Kind::Net => self.parse_net()?,
                token::Kind::Transition => self.parse_transition()?,
                token::Kind::Place => self.parse_place()?,
                token::Kind::Note => self.parse_note()?,
                token::Kind::Label => self.parse_label()?,
                token::Kind::Priority => self.parse_priority()?,
                token::Kind::EndOfFile => break,
                _ => {
                    return Err(Box::new(ParserError::UnexpectedToken(
                        token,
                        "This token can not start a line".to_string(),
                    )));
                }
            }
        }
        Ok(self.net)
    }

    /// Create a new parser from reader
    ///
    /// ```ignore
    /// let parser = Parser::new(&"net Réseau\ntr t0 p0 -> p1\ntr t1 p1 -> p0".as_bytes());
    /// ```
    pub fn new(reader: R) -> Self {
        Self {
            lexer: Lexer::new(reader),
            net: Net::default(),
        }
    }

    /// Get transition from net or create one
    fn get_or_create_transition(&mut self, name: &str) -> Result<TransitionId, Box<dyn Error>> {
        match self.net.get_index_by_name(name) {
            Some(NodeId::Transition(id)) => Ok(id),
            Some(NodeId::Place(_)) => Err(Box::new(NetError::DuplicatedName(name.to_string()))),
            None => {
                let tr = self.net.create_transition();
                self.net.rename_node(NodeId::Transition(tr), name)?;
                Ok(tr)
            }
        }
    }

    /// Get place from net or create one
    fn get_or_create_place(&mut self, name: &str) -> Result<PlaceId, Box<dyn Error>> {
        match self.net.get_index_by_name(name) {
            Some(NodeId::Place(id)) => Ok(id),
            Some(NodeId::Transition(_)) => {
                Err(Box::new(NetError::DuplicatedName(name.to_string())))
            }
            None => {
                let pl = self.net.create_place();
                self.net.rename_node(NodeId::Place(pl), name)?;
                Ok(pl)
            }
        }
    }

    /// Parse the label token
    fn parse_label(&mut self) -> Result<(), Box<dyn Error>> {
        self.lexer.read()?;
        let token = self.lexer.read()?;
        match token.kind {
            Kind::Identifier(identifier) => {
                let index = match self.net.get_index_by_name(&identifier) {
                    None => return Err(Box::new(NetError::UnknownIdentifier(identifier))),
                    Some(index) => index,
                };
                let token = self.lexer.read()?;
                match token.kind {
                    Kind::Identifier(identifier) => {
                        match index {
                            NodeId::Place(pl) => self.net[pl].label = Some(identifier),
                            NodeId::Transition(tr) => self.net[tr].label = Some(identifier),
                        }
                        Ok(())
                    }

                    _ => Err(Box::new(ParserError::UnexpectedToken(
                        token,
                        "Expected TokenKind::Identifier(_)".to_string(),
                    ))),
                }
            }
            _ => Err(Box::new(ParserError::UnexpectedToken(
                token,
                "Expected TokenKind::Identifier(_)".to_string(),
            ))),
        }
    }

    /// Parse the net token
    fn parse_net(&mut self) -> Result<(), Box<dyn Error>> {
        self.lexer.read()?;
        let identifier = self.lexer.read()?;
        match identifier.kind {
            token::Kind::Identifier(id) => {
                self.net.name = id;
                Ok(())
            }
            _ => Err(Box::new(ParserError::UnexpectedToken(
                identifier,
                "Expected TokenKind::Identifier(_)".to_string(),
            ))),
        }
    }

    /// Parse a transition line
    fn parse_transition(&mut self) -> Result<(), Box<dyn Error>> {
        self.lexer.read()?;
        let transition_name = match self.lexer.read()?.kind {
            token::Kind::Identifier(name) => name,
            _ => {
                return Err(Box::new(ParserError::UnexpectedToken(
                    self.lexer.current_token.clone(),
                    "Expected TokenKind::Identifier(_)".to_string(),
                )));
            }
        };

        let tr = self.get_or_create_transition(&transition_name)?;

        // Try to read label
        if self.lexer.peek()?.kind == token::Kind::InlineLabel {
            self.lexer.read()?;
            match self.lexer.read()?.kind {
                token::Kind::Identifier(label) => self.net[tr].label = Some(label),
                _ => {
                    return Err(Box::new(ParserError::UnexpectedToken(
                        self.lexer.current_token.clone(),
                        "Expected TokenKind::Identifier(_)".to_string(),
                    )));
                }
            }
        }

        // Try to read interval
        if let token::Kind::TimeInterval(start, end) = self.lexer.peek()?.kind {
            self.lexer.read()?;
            self.net[tr].time = self.net[tr].time.intersect(TimeRange { start, end })
        }

        if matches!(
            self.lexer.peek()?.kind,
            token::Kind::Identifier(_) | token::Kind::Arrow
        ) {
            // Try to read input places
            loop {
                let identifier = &match self.lexer.peek()?.kind {
                    token::Kind::Identifier(name) => name,
                    _ => break,
                };
                let pl = self.get_or_create_place(identifier)?;
                self.lexer.read()?;

                let new_arc = self.parse_transition_input_arc(pl, tr)?;
                self.net.add_arc(new_arc)?;
            }

            self.parse_arrow()?;

            // Try to read output
            loop {
                let identifier = &match self.lexer.peek()?.kind {
                    token::Kind::Identifier(name) => name,
                    _ => break,
                };
                let pl = self.get_or_create_place(identifier)?;
                self.lexer.read()?;
                let new_arc = self.parse_transition_output_arc(pl, tr)?;
                self.net.add_arc(new_arc)?;
            }
        }

        Ok(())
    }

    /// Try to parse int
    fn parse_int(&mut self) -> Result<usize, Box<dyn Error>> {
        Ok(match self.lexer.peek()?.kind {
            token::Kind::Int(value) => {
                self.lexer.read()?;
                value
            }
            _ => {
                return Err(Box::new(ParserError::UnexpectedToken(
                    self.lexer.current_token.clone(),
                    "Expected TokenKind::Int(_)".to_string(),
                )));
            }
        })
    }

    /// Try to parse arrow
    fn parse_arrow(&mut self) -> Result<(), Box<dyn Error>> {
        if self.lexer.read()?.kind == token::Kind::Arrow {
            Ok(())
        } else {
            Err(Box::new(ParserError::UnexpectedToken(
                self.lexer.current_token.clone(),
                "Expected TokenKind::Arrow".to_string(),
            )))
        }
    }
    fn parse_transition_input_arc(
        &mut self,
        place: PlaceId,
        transition: TransitionId,
    ) -> Result<arc::Kind, Box<dyn Error>> {
        match self.lexer.peek()?.kind {
            token::Kind::NormalArc => {
                self.lexer.read()?;
                Ok(arc::Kind::Consume(place, transition, self.parse_int()?))
            }
            token::Kind::InhibitorArc => {
                self.lexer.read()?;
                Ok(arc::Kind::Inhibitor(place, transition, self.parse_int()?))
            }
            token::Kind::TestArc => {
                self.lexer.read()?;
                Ok(arc::Kind::Test(place, transition, self.parse_int()?))
            }
            token::Kind::StopWatchArc => {
                self.lexer.read()?;
                Ok(arc::Kind::StopWatch(place, transition, self.parse_int()?))
            }
            token::Kind::StopWatchInhibitorArc => {
                self.lexer.read()?;
                Ok(arc::Kind::StopWatchInhibitor(
                    place,
                    transition,
                    self.parse_int()?,
                ))
            }
            token::Kind::Arrow
            | token::Kind::EndOfFile
            | token::Kind::Identifier(_)
            | token::Kind::NewLine => Ok(arc::Kind::Consume(place, transition, 1)),
            _ => Err(Box::new(ParserError::UnexpectedToken(
                self.lexer.current_token.clone(),
                "Expected TokenKind::Arc*".to_string(),
            ))),
        }
    }

    fn parse_transition_output_arc(
        &mut self,
        place: PlaceId,
        transition: TransitionId,
    ) -> Result<arc::Kind, Box<dyn Error>> {
        match self.lexer.peek()?.kind {
            token::Kind::NormalArc => {
                self.lexer.read()?;
                Ok(arc::Kind::Produce(place, transition, self.parse_int()?))
            }
            arc @ token::Kind::InhibitorArc
            | arc @ token::Kind::TestArc
            | arc @ token::Kind::StopWatchArc
            | arc @ token::Kind::StopWatchInhibitorArc => Err(Box::new(
                ParserError::UnexpectedArc(self.lexer.current_token.position, arc),
            )),
            token::Kind::Arrow
            | token::Kind::EndOfFile
            | token::Kind::Identifier(_)
            | token::Kind::NewLine => Ok(arc::Kind::Produce(place, transition, 1)),
            _ => Err(Box::new(ParserError::UnexpectedToken(
                self.lexer.current_token.clone(),
                "Expected TokenKind::Arc*".to_string(),
            ))),
        }
    }

    /// Parse a place line
    fn parse_place(&mut self) -> Result<(), Box<dyn Error>> {
        self.lexer.read()?;
        let pl = match self.lexer.read()?.kind {
            token::Kind::Identifier(name) => self.get_or_create_place(&name)?,
            _ => {
                return Err(Box::new(ParserError::UnexpectedToken(
                    self.lexer.current_token.clone(),
                    "Expected TokenKind::Identifier(_)".to_string(),
                )));
            }
        };

        // Parse Label
        if self.lexer.peek()?.kind == token::Kind::InlineLabel {
            self.lexer.read()?;
            match self.lexer.read()?.kind {
                token::Kind::Identifier(label) => {
                    self.net[pl].label = Some(label);
                }
                _ => {
                    return Err(Box::new(ParserError::UnexpectedToken(
                        self.lexer.current_token.clone(),
                        "Expected TokenKind::Identifier(_)".to_string(),
                    )));
                }
            }
        }

        // Parse marking
        if let token::Kind::Int(v) = self.lexer.peek()?.kind {
            self.lexer.read()?;
            self.net[pl].initial += v;
        }

        if matches!(
            self.lexer.peek()?.kind,
            token::Kind::Identifier(_) | token::Kind::Arrow
        ) {
            // Parse inputs
            loop {
                let identifier = &match self.lexer.peek()?.kind {
                    token::Kind::Identifier(name) => name,
                    _ => break,
                };
                let tr = self.get_or_create_transition(identifier)?;
                self.lexer.read()?;
                let new_arc = self.parse_transition_output_arc(pl, tr)?;
                self.net.add_arc(new_arc)?;
            }

            self.parse_arrow()?;

            // Parse output
            loop {
                let identifier = &match self.lexer.peek()?.kind {
                    token::Kind::Identifier(name) => name,
                    _ => break,
                };
                let tr = self.get_or_create_transition(identifier)?;
                self.lexer.read()?;
                let new_arc = self.parse_transition_input_arc(pl, tr)?;
                self.net.add_arc(new_arc)?;
            }
        }

        Ok(())
    }

    /// Parse a note line
    fn parse_note(&mut self) -> Result<(), Box<dyn Error>> {
        self.lexer.read()?;
        match self.lexer.read()?.kind {
            token::Kind::Identifier(_) => {}
            _ => {
                return Err(Box::new(ParserError::UnexpectedToken(
                    self.lexer.current_token.clone(),
                    "Expected TokenKind::Identifier(_)".to_string(),
                )));
            }
        }
        self.parse_int()?;
        match self.lexer.read()?.kind {
            token::Kind::Identifier(_) => {}
            _ => {
                return Err(Box::new(ParserError::UnexpectedToken(
                    self.lexer.current_token.clone(),
                    "Expected TokenKind::Identifier(_)".to_string(),
                )));
            }
        }
        Ok(())
    }

    /// Parse a priority line
    fn parse_priority(&mut self) -> Result<(), Box<dyn Error>> {
        self.lexer.read()?;
        let mut pre = vec![];
        let mut post = vec![];
        while let token::Kind::Identifier(id) = self.lexer.peek()?.kind {
            self.lexer.read()?;
            pre.push(self.get_or_create_transition(&id)?);
        }
        let order = match self.lexer.read()?.kind {
            token::Kind::GreaterThan => false,
            token::Kind::LessThan => true,
            _ => {
                return Err(Box::new(ParserError::UnexpectedToken(
                    self.lexer.current_token.clone(),
                    "Expected TokenKind::GreaterThan or TokenKind::LessThan".to_string(),
                )));
            }
        };
        while let token::Kind::Identifier(id) = self.lexer.peek()?.kind {
            self.lexer.read()?;
            post.push(self.get_or_create_transition(&id)?);
        }

        let (pre, post) = if order { (post, pre) } else { (pre, post) };
        for id in &pre {
            for id_post in &post {
                self.net.add_priority(*id, *id_post);
            }
        }
        Ok(())
    }
}
