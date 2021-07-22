use std::error::Error;
use std::ops::{Index, IndexMut};

use indexed_vec::IndexVec;

use crate::standard::{Place, Transition};
use crate::{arc, timed, NetError, PlaceId, TransitionId};

/// Standard petri network, with only produce and consume arcs
///
/// This structure is indexed with [`PlaceId`] and [`TransitionId`] to allow easy access to places
/// and transitions.
///
/// As this kind of network is a subset of timed petri net, so we can create one from timed petri
/// net (but you loose [`arc::Kind::Inhibitor`], [`arc::Kind::StopWatch`] and
/// [`arc::Kind::StopWatchInhibitor`] arcs and timings).
#[derive(Default, Debug, Clone)]
pub struct Net {
    /// Name of this network
    pub name: String,
    /// Transitions of the network
    pub transitions: IndexVec<TransitionId, Transition>,
    /// Places of the network
    pub places: IndexVec<PlaceId, Place>,
}

impl Index<TransitionId> for Net {
    type Output = Transition;

    fn index(&self, index: TransitionId) -> &Self::Output {
        &self.transitions[index]
    }
}

impl Index<PlaceId> for Net {
    type Output = Place;

    fn index(&self, index: PlaceId) -> &Self::Output {
        &self.places[index]
    }
}

impl IndexMut<TransitionId> for Net {
    fn index_mut(&mut self, index: TransitionId) -> &mut Self::Output {
        self.transitions.index_mut(index)
    }
}

impl IndexMut<PlaceId> for Net {
    fn index_mut(&mut self, index: PlaceId) -> &mut Self::Output {
        self.places.index_mut(index)
    }
}

impl From<timed::Net> for Net {
    fn from(timed: timed::Net) -> Self {
        // Crate a new network
        let mut net = Net {
            name: timed.name,
            ..Net::default()
        };
        // Copy all places
        for place in timed.places {
            let i = net.create_place();
            net[i].name = place.name;
            net[i].initial = place.initial;
            net[i].label = place.label;
        }

        // Copy transitions and arcs from timed petri net
        for transition in timed.transitions {
            let new_tr = net.create_transition();
            net[new_tr].name = transition.name;
            net[new_tr].label = transition.label;
            for &(pl, weight) in transition.consume.iter() {
                net.add_arc(arc::Kind::Consume(pl, net[new_tr].id, weight as usize))
                    .unwrap();
            }
            for &(pl, weight) in transition.produce.iter() {
                net.add_arc(arc::Kind::Produce(pl, net[new_tr].id, weight as usize))
                    .unwrap();
            }
            for &(pl, weight) in transition.conditions.iter() {
                // Replace condition arc with a couple of [arc::Kind::Consume] and [arc::Kind::Produce] arcs.
                net.add_arc(arc::Kind::Consume(pl, net[new_tr].id, weight as usize))
                    .unwrap();
                net.add_arc(arc::Kind::Produce(pl, net[new_tr].id, weight as usize))
                    .unwrap();
            }
        }
        net
    }
}

impl Net {
    /// Create a place in the network without name and return its index
    pub fn create_place(&mut self) -> PlaceId {
        self.places.push(Place {
            id: PlaceId::from(self.places.len()),
            ..Place::default()
        });
        self.places.last_idx().unwrap()
    }

    /// Create a transition in the network without name and return its index
    pub fn create_transition(&mut self) -> TransitionId {
        self.transitions.push(Transition {
            id: TransitionId::from(self.transitions.len()),
            ..Transition::default()
        });
        self.transitions.last_idx().unwrap()
    }

    /// Search a place by its name
    pub fn search_place_by_name(&self, name: &str) -> Option<PlaceId> {
        self.places
            .iter()
            .position(|v| v.name == name)
            .map(PlaceId::from)
    }

    /// Search a transition by its name
    pub fn search_transition_by_name(&self, name: &str) -> Option<TransitionId> {
        self.transitions
            .iter()
            .position(|v| v.name == name)
            .map(TransitionId::from)
    }

    /// Add an arc in the network. This kind of network support only [`arc::Kind::Consume`] and
    /// [`arc::Kind::Produce`] arcs.
    ///
    /// # Errors
    /// Return [`NetError::UnsupportedArc`] when trying to add a kind of arc which is not supported
    pub fn add_arc(&mut self, arc: arc::Kind) -> Result<(), Box<dyn Error>> {
        match arc {
            arc::Kind::Consume(pl_id, tr_id, w) => {
                self.transitions[tr_id].consume.insert_or_add(pl_id, w);
                self.places[pl_id].consumed_by.insert_or_add(tr_id, w);
                Ok(())
            }
            arc::Kind::Produce(pl_id, tr_id, w) => {
                self.transitions[tr_id].produce.insert_or_add(pl_id, w);
                self.places[pl_id].produced_by.insert_or_add(tr_id, w);
                Ok(())
            }
            a => Err(Box::new(NetError::UnsupportedArc(a))),
        }
    }

    /// Disconnect a place in the network
    ///
    /// The place is not really deleted to avoid memory relocation and extra information about
    /// this place such as name can be useful later.
    pub fn delete_place(&mut self, place: PlaceId) {
        for &(tr, _) in self.places[place].consumed_by.iter() {
            self.transitions[tr].consume.delete(place);
        }
        for &(tr, _) in self.places[place].produced_by.iter() {
            self.transitions[tr].produce.delete(place);
        }
        self.places[place].consumed_by.clear();
        self.places[place].produced_by.clear();
        self.places[place].deleted = true;
    }

    /// Disconnect a transition in the network
    ///
    /// The transition is not really deleted to avoid memory relocation and extra information about
    /// this transitions such as name can be useful later.
    pub fn delete_transition(&mut self, transition: TransitionId) {
        for &(pl, _) in self.transitions[transition].consume.iter() {
            self.places[pl].consumed_by.delete(transition);
        }

        for &(pl, _) in self.transitions[transition].produce.iter() {
            self.places[pl].produced_by.delete(transition);
        }

        self.transitions[transition].consume.clear();
        self.transitions[transition].produce.clear();
        self.transitions[transition].deleted = true;
    }

    /// Clone an existing place with all its arcs
    pub fn clone_place(&mut self, old_pl: PlaceId) -> PlaceId {
        let new_pl = self.create_place();

        // Duplication des transitions
        for old_arc in self[old_pl].get_arcs() {
            match old_arc {
                arc::Kind::Consume(_, tr, w) => {
                    self.add_arc(arc::Kind::Consume(new_pl, tr, w)).unwrap();
                }
                arc::Kind::Produce(_, tr, w) => {
                    self.add_arc(arc::Kind::Produce(new_pl, tr, w)).unwrap();
                }
                _ => {}
            }
        }

        self.places[new_pl].initial = self.places[old_pl].initial;
        self.places[new_pl].deleted = self.places[old_pl].deleted;
        new_pl
    }

    /// Rename all places and transition with the name `pl_{idx}`
    ///
    /// If the place or the transition doesn't have a label, the old name is copied to label.
    pub fn auto_name(&mut self) {
        for (pl, place) in self.places.iter_mut().enumerate() {
            if place.label.is_empty() {
                place.label = place.name.clone()
            }
            place.name = format!("pl_{}", pl);
        }

        for (tr, transition) in self.transitions.iter_mut().enumerate() {
            if transition.label.is_empty() {
                transition.label = transition.name.clone()
            }
            transition.name = format!("tr_{}", tr);
        }
    }

    /// Create a new network without all disconected nodes and without labels to avoid extra memory
    /// consumption.
    ///
    /// It returns a new network and the mapping between old indexes and new indexes.
    #[must_use]
    pub fn new_without_disconnected(
        &self,
    ) -> (
        Net,
        IndexVec<TransitionId, TransitionId>,
        IndexVec<PlaceId, PlaceId>,
    ) {
        let mut new = Self::default();
        let mut transition_map = IndexVec::<TransitionId, TransitionId>::default();
        let mut place_map = IndexVec::<PlaceId, PlaceId>::default(); // map[new_pl] = old_pl
        let mut place_map_inv = IndexVec::<PlaceId, PlaceId>::default(); //map_inv[old_pl] = new_pl

        for (old_pl, old_place) in self.places.iter_enumerated() {
            place_map_inv.push(PlaceId::from(place_map.len()));
            if !old_place.deleted && !old_place.is_disconnected() {
                let pl = new.create_place();
                new.places[pl].initial = old_place.initial;
                new.places[pl].name = format!("pl_{:0}", place_map.len());
                place_map.push(old_pl);
            }
        }

        for (old_tr, old_transition) in self.transitions.iter_enumerated() {
            if !old_transition.is_disconnected() {
                let tr = new.create_transition();
                new.transitions[tr].name = format!("tr_{:0}", transition_map.len());
                for &(pl, w) in old_transition.produce.iter() {
                    new.add_arc(arc::Kind::Produce(
                        place_map_inv[pl],
                        TransitionId::from(transition_map.len()),
                        w as usize,
                    ))
                    .unwrap();
                }
                for &(pl, w) in old_transition.consume.iter() {
                    new.add_arc(arc::Kind::Consume(
                        place_map_inv[pl],
                        TransitionId::from(transition_map.len()),
                        w as usize,
                    ))
                    .unwrap();
                }
                transition_map.push(old_tr);
            }
        }

        (new, transition_map, place_map)
    }
}
