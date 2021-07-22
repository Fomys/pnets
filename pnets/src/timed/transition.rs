use crate::timed::TimeRange;
use crate::{Marking, PlaceId, TransitionId};

/// Represent a transition in the network
#[derive(Debug, Default)]
pub struct Transition {
    pub(crate) id: TransitionId,
    /// Name of the transition
    pub name: String,
    /// Label of the transition
    pub label: String,
    /// Timerange of the transition
    pub time: TimeRange,

    /// Conditions of transition
    pub conditions: Marking<PlaceId>,
    /// Inhibitors of the transition
    pub inhibitors: Marking<PlaceId>,

    /// Consumption of the transition
    pub consume: Marking<PlaceId>,
    /// Production of the transition
    pub produce: Marking<PlaceId>,

    /// Priorities of the transitions (this transition must be activated before all transition in this vector)
    pub priorities: Vec<TransitionId>,
}

impl Transition {
    /// Returns the id of the transition
    #[must_use]
    pub fn id(&self) -> TransitionId {
        self.id
    }

    /// Returns [`true`] if this transition is disconnected from the network
    #[must_use]
    pub fn is_disconnected(&self) -> bool {
        self.produce.is_empty()
            && self.consume.is_empty()
            && self.conditions.is_empty()
            && self.inhibitors.is_empty()
    }
}
