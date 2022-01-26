use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use crate::{arc, PlaceId, TransitionId};

/// Errors generated when manipulating a Petri net
#[derive(Debug, Eq, PartialEq)]
pub enum NetError {
    /// There is at least one cyclic priority (t1 > t0 > t1 for example)
    CyclicPriorities,
    /// There is an invalid time range in the net
    InvalidTimeRange,
    /// This kind of arc is not supported in ths version of the crate
    UnsupportedArc(arc::Kind),
    /// This transition id is invalid
    InvalidTransition(TransitionId),
    /// This place id is invalid
    InvalidPlace(PlaceId),
    /// There is a duplicated name in the net
    DuplicatedName(String),
    /// This identifier is not found in the net
    UnknownIdentifier(String),
    /// This net contains an invalid arc (place to place, transition to transition, ...)
    InvalidArc,
}

impl fmt::Display for NetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NetError::CyclicPriorities => write!(f, "Cyclic priorities in Petri net"),
            NetError::InvalidTimeRange => write!(f, "Invalid time range found"),
            NetError::UnsupportedArc(arc) => {
                write!(f, "Unsupported arc {}", arc)
            }
            NetError::InvalidTransition(tr) => write!(f, "Invalid transition id {}", tr),
            NetError::InvalidPlace(pl) => write!(f, "Invalid place id {}", pl),
            NetError::DuplicatedName(name) => {
                write!(f, "You try to create duplicated name in the net: {}", name)
            }
            NetError::UnknownIdentifier(identifier) => {
                write!(f, "Identifier {} not found in the net.", identifier)
            }
            NetError::InvalidArc => {
                write!(f, "Invalid arc in the net")
            }
        }
    }
}

impl Error for NetError {}
