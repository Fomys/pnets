use crate::{arc, Marking, PlaceId, TransitionId};

/// Place with only producers and consumers
#[derive(Default, Debug, Clone)]
pub struct Place {
    /// Identifier of the place
    pub(crate) id: PlaceId,
    /// Label of the place
    pub label: Option<String>,
    /// Initial value of the place
    pub initial: usize,

    /// Transitions that produce this place
    pub produced_by: Marking<TransitionId>,
    /// Transitions that consume this place
    pub consumed_by: Marking<TransitionId>,

    /// This place is disconnected from the net and only kept to avoid index problems
    pub deleted: bool,
}

impl Place {
    /// Returns the id of the place
    #[must_use]
    pub fn id(&self) -> PlaceId {
        self.id
    }

    /// Returns [`true`] if this place is disconnected from the net
    #[must_use]
    pub fn is_disconnected(&self) -> bool {
        self.produced_by.is_empty() && self.consumed_by.is_empty()
    }

    /// Generate a vector over all arcs connected to this place
    #[must_use]
    pub fn get_arcs(&self) -> Vec<arc::Kind> {
        let mut arcs = vec![];
        for &(tr, w) in self.consumed_by.iter() {
            arcs.push(arc::Kind::Consume(self.id, tr, w))
        }
        for &(tr, w) in self.produced_by.iter() {
            arcs.push(arc::Kind::Produce(self.id, tr, w))
        }
        arcs
    }
}
