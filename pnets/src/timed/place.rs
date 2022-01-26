use crate::{Marking, PlaceId, TransitionId};

/// Representation of a timed place
#[derive(Debug, Default)]
pub struct Place {
    /// Id of this place
    pub(crate) id: PlaceId,
    /// Label of the place
    pub label: Option<String>,
    /// Initial value of the place
    pub initial: usize,

    /// Transitions that produce this place
    pub produced_by: Marking<TransitionId>,
    /// Transitions that consume this place
    pub consumed_by: Marking<TransitionId>,

    /// Transitions that has condition on this place
    pub condition_for: Marking<TransitionId>,
    /// Transitions that has inhibitor on this place
    pub inhibitor_for: Marking<TransitionId>,
}

impl Place {
    /// Return the id of the place
    #[must_use]
    pub fn id(&self) -> PlaceId {
        self.id
    }

    /// Disconnect a place from the net
    #[must_use]
    pub fn is_disconnected(&self) -> bool {
        self.produced_by.is_empty()
            && self.consumed_by.is_empty()
            && self.condition_for.is_empty()
            && self.inhibitor_for.is_empty()
    }
}
