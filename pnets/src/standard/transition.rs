use crate::{arc, Marking, PlaceId, TransitionId};

/// Transition with only production and consumption
#[derive(Default, Debug, Clone)]
pub struct Transition {
    /// Identifier of the transition
    pub(crate) id: TransitionId,
    /// Label of the transition
    pub label: Option<String>,

    /// Consumption of the transition
    pub consume: Marking<PlaceId>,
    /// Production of the transition
    pub produce: Marking<PlaceId>,

    /// This transition is disconnected from the network and only kept to avoid index problems
    pub deleted: bool,
}

impl Transition {
    /// Return the id of the transition
    #[must_use]
    pub fn id(&self) -> TransitionId {
        self.id
    }

    /// Returns [`true`] if this transition is disconnected from the network
    #[must_use]
    pub fn is_disconnected(&self) -> bool {
        self.consume.is_empty() && self.produce.is_empty()
    }

    /// Get all arcs of this transition
    #[must_use]
    pub fn get_arcs(&self) -> Vec<arc::Kind> {
        let mut arcs = vec![];
        for &(pl, w) in self.consume.iter() {
            arcs.push(arc::Kind::Consume(pl, self.id, w))
        }
        for &(pl, w) in self.produce.iter() {
            arcs.push(arc::Kind::Produce(pl, self.id, w))
        }
        arcs
    }
}
