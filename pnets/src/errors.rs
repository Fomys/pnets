use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

use crate::{arc, PlaceId, TransitionId};

/// Errors generated when manipulating a Petri net
#[derive(Debug)]
pub enum NetError {
    /// There is at least one cyclic priority (t1 > t0 > t1 for example)
    CyclicPriorities,
    /// There is an invalid time range in the network
    InvalidTimeRange,
    /// This kind of arc is not supported in ths version of the crate
    UnsupportedArc(arc::Kind),
    /// This transition id is invalid
    InvalidTransition(TransitionId),
    /// This place id is invalid
    InvalidPlace(PlaceId),
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
        }
    }
}

impl Error for NetError {}
