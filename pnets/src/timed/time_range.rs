//! Time range for timed net
use std::fmt;
use std::fmt::Formatter;

/// Type of bound for time range
#[derive(PartialEq, Debug, Clone, Eq, Copy)]
pub enum Bound {
    /// Closed interval bound
    Closed(usize),
    /// Open interval bound
    Open(usize),
    /// Infinity bound
    Infinity,
}

impl fmt::Display for Bound {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Bound::Closed(v) => write!(f, "Close({})", v),
            Bound::Open(v) => write!(f, "Open({})", v),
            Bound::Infinity => write!(f, "Infinity"),
        }
    }
}

/// Represent a timerange in the petri network
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct TimeRange {
    /// Start of the time range
    pub start: Bound,
    /// end of the time range
    pub end: Bound,
}

impl TimeRange {
    /// Return a new timerange corresponding to the intersection of the timeranges
    #[must_use]
    pub fn intersect(&self, other: Self) -> Self {
        let new_start = match (self.start, other.start) {
            (Bound::Open(self_value), Bound::Open(other_value))
            | (Bound::Closed(self_value), Bound::Open(other_value))
            | (Bound::Open(self_value), Bound::Closed(other_value))
            | (Bound::Closed(self_value), Bound::Closed(other_value))
                if self_value != other_value =>
            {
                if self_value < other_value {
                    other.start
                } else {
                    self.start
                }
            }
            (Bound::Closed(value), Bound::Closed(_)) => Bound::Closed(value),
            (Bound::Open(value), _) | (_, Bound::Open(value)) => Bound::Open(value),
            (Bound::Infinity, o) | (o, Bound::Infinity) => o,
        };

        let new_end = match (self.end, other.end) {
            (Bound::Open(self_value), Bound::Open(other_value))
            | (Bound::Closed(self_value), Bound::Open(other_value))
            | (Bound::Open(self_value), Bound::Closed(other_value))
            | (Bound::Closed(self_value), Bound::Closed(other_value))
                if self_value != other_value =>
            {
                if self_value > other_value {
                    other.end
                } else {
                    self.end
                }
            }
            (Bound::Closed(value), Bound::Closed(_)) => Bound::Closed(value),
            (Bound::Open(value), _) | (_, Bound::Open(value)) => Bound::Open(value),
            (Bound::Infinity, o) | (o, Bound::Infinity) => o,
        };
        TimeRange {
            start: new_start,
            end: new_end,
        }
    }
}

impl Default for TimeRange {
    fn default() -> Self {
        Self {
            start: Bound::Closed(0),
            end: Bound::Infinity,
        }
    }
}
