//! Arcs which are supported by this framework
use std::fmt;
use std::fmt::Formatter;

use crate::net::{PlaceId, TransitionId};

/// All kind of arc which can be inserted in the Ptri net
///
/// All arcs have is stored with (destination, source, weight)
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    /// Consuming arc
    Consume(PlaceId, TransitionId, usize),
    /// Producer arc
    Produce(PlaceId, TransitionId, usize),
    /// Test arc
    Test(PlaceId, TransitionId, usize),
    /// Inhibitor arc
    Inhibitor(PlaceId, TransitionId, usize),
    /// Stopwatch arc
    StopWatch(PlaceId, TransitionId, usize),
    /// Stopwatch inhibitor arc
    StopWatchInhibitor(PlaceId, TransitionId, usize),
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Kind::Consume(tr_id, pl_id, w) => {
                write!(f, "consume {} from {} for transition {}", w, pl_id, tr_id)
            }
            Kind::Produce(pl_id, tr_id, w) => {
                write!(f, "produce {} in {} for transition {}", w, pl_id, tr_id)
            }
            Kind::Test(pl_id, tr_id, w) => {
                write!(f, "test {} from {} for transition {}", w, pl_id, tr_id)
            }
            Kind::Inhibitor(pl_id, tr_id, w) => {
                write!(
                    f,
                    "Inhibit transition {} when {} contains more than {}",
                    tr_id, pl_id, w
                )
            }
            Kind::StopWatch(pl_id, tr_id, w) => {
                write!(
                    f,
                    "stopwatch on place {} with value {} for transition {}",
                    pl_id, w, tr_id
                )
            }
            Kind::StopWatchInhibitor(pl_id, tr_id, w) => {
                write!(
                    f,
                    "stopwatch inhibitor on place {} with value {} for transition {}",
                    pl_id, w, tr_id
                )
            }
        }
    }
}
