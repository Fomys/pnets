use std::error::Error;
use std::ops::{Index, IndexMut};

use indexed_vec::IndexVec;

use crate::net::NodeId;
use crate::timed::{Place, Transition};
use crate::{arc, standard, NetError, PlaceId, TransitionId};
use bimap::BiMap;

/// Timed Petri net, with produce, consume, condition and inhibitors arcs
///
/// This structure is indexed with [`PlaceId`] and [`TransitionId`] to allow easy access to places
/// and transitions.
///
/// As this kind of net is a superset of standard Petri net, we can create one from standard
/// Petri net without loosing any informations.
#[derive(Debug, Default)]
pub struct Net {
    /// Name of this net
    pub name: String,
    /// BiHashmap to get id from index and index from id
    id_index_map: BiMap<String, NodeId>,
    /// Prefix for new places and transitions
    automatic_prefix: String,
    /// Transitions of the net
    pub transitions: IndexVec<TransitionId, Transition>,
    /// Places of the net
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

impl From<&standard::Net> for Net {
    fn from(standard: &standard::Net) -> Self {
        // Crate a new net
        let mut net = Net {
            name: standard.name.clone(),
            ..Net::default()
        };

        // Copy all places
        for place in &standard.places {
            let new_pl = net.create_place();
            net[new_pl].initial = place.initial;
            net[new_pl].label = place.label.clone();
            net.rename_node(
                NodeId::Place(place.id),
                &standard
                    .get_name_by_index(&NodeId::Place(place.id))
                    .unwrap(),
            )
            .unwrap();
        }

        // Copy transitions and arcs from timed Petri nets
        for transition in &standard.transitions {
            let new_tr = net.create_transition();
            net[new_tr].label = transition.label.clone();
            for &(pl, weight) in transition.consume.iter() {
                net.add_arc(arc::Kind::Consume(pl, net[new_tr].id, weight as usize))
                    .unwrap();
            }
            for &(pl, weight) in transition.produce.iter() {
                net.add_arc(arc::Kind::Produce(pl, net[new_tr].id, weight as usize))
                    .unwrap();
            }
            net.rename_node(
                NodeId::Transition(transition.id),
                &standard
                    .get_name_by_index(&NodeId::Transition(transition.id))
                    .unwrap(),
            )
            .unwrap();
        }
        net
    }
}

impl Net {
    /// Create a place with automatic name
    pub fn create_place(&mut self) -> PlaceId {
        self.places.push(Place {
            id: PlaceId::from(self.places.len()),
            ..Place::default()
        });
        self.id_index_map.insert(
            format!("{}{}", self.automatic_prefix, self.id_index_map.len()),
            NodeId::Place(self.places.last_idx().unwrap()),
        );
        self.places.last_idx().unwrap()
    }

    /// Create a transition with an empty name
    pub fn create_transition(&mut self) -> TransitionId {
        self.transitions.push(Transition {
            id: TransitionId::from(self.transitions.len()),
            ..Transition::default()
        });
        self.id_index_map.insert(
            format!("{}{}", self.automatic_prefix, self.id_index_map.len()),
            NodeId::Transition(self.transitions.last_idx().unwrap()),
        );
        self.transitions.last_idx().unwrap()
    }

    /// Get node name with its id
    pub fn get_name_by_index(&self, index: &NodeId) -> Option<String> {
        self.id_index_map.get_by_right(index).map(|v| v.clone())
    }

    /// Get node id with its name
    pub fn get_index_by_name(&self, name: &str) -> Option<NodeId> {
        self.id_index_map.get_by_left(name).map(|v| *v)
    }

    /// Rename node
    pub fn rename_node(&mut self, id: NodeId, name: &str) -> Result<(), NetError> {
        if name.starts_with(&self.automatic_prefix) {
            self.automatic_prefix.push('a');
        }
        match self.id_index_map.get_by_left(name) {
            None => {
                self.id_index_map.remove_by_right(&id);
                self.id_index_map.insert(name.to_string(), id);
                Ok(())
            }
            Some(&nid) if nid == id => Ok(()),
            Some(_) => Err(NetError::DuplicatedName(name.to_string())),
        }
    }

    /// Add an arc in the net. This kind of net support only [`arc::Kind::Consume`],
    /// [`arc::Kind::Produce`], [`arc::Kind::Inhibitor`] and [`arc::Kind::Test`] arcs.
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
            arc::Kind::Test(pl_id, tr_id, w) => {
                self.transitions[tr_id].conditions.insert_or_max(pl_id, w);
                self.places[pl_id].condition_for.insert_or_max(tr_id, w);
                Ok(())
            }
            arc::Kind::Inhibitor(pl_id, tr_id, w) => {
                self.transitions[tr_id].inhibitors.insert_or_min(pl_id, w);
                self.places[pl_id].inhibitor_for.insert_or_min(tr_id, w);
                Ok(())
            }
            a => Err(Box::new(NetError::UnsupportedArc(a))),
        }
    }

    /// Disconnect a place in the net
    ///
    /// The place is not really deleted to avoid memory relocation and extra information about
    /// this place such as name can be useful later.
    pub fn delete_place(&mut self, place: PlaceId) {
        for &(tr, _) in self.places[place].consumed_by.iter() {
            self.transitions[tr].consume.delete(place);
        }
        for &(tr, _) in self.places[place].condition_for.iter() {
            self.transitions[tr].conditions.delete(place);
        }
        for &(tr, _) in self.places[place].inhibitor_for.iter() {
            self.transitions[tr].inhibitors.delete(place);
        }
        for &(tr, _) in self.places[place].produced_by.iter() {
            self.transitions[tr].produce.delete(place);
        }
        self.places[place].consumed_by.clear();
        self.places[place].condition_for.clear();
        self.places[place].inhibitor_for.clear();
        self.places[place].produced_by.clear();
    }

    /// Disconnect a transition in the net
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

        for &(pl, _) in self.transitions[transition].inhibitors.iter() {
            self.places[pl].inhibitor_for.delete(transition);
        }

        for &(pl, _) in self.transitions[transition].conditions.iter() {
            self.places[pl].condition_for.delete(transition);
        }
        self.transitions[transition].consume.clear();
        self.transitions[transition].produce.clear();
        self.transitions[transition].priorities.clear();
        self.transitions[transition].inhibitors.clear();
        self.transitions[transition].conditions.clear();
    }

    /// Add a priority relation in the net
    pub fn add_priority(&mut self, tr_index: TransitionId, over: TransitionId) {
        match self.transitions[tr_index].priorities.binary_search(&over) {
            Ok(_) => {} // element already in vector @ `pos`
            Err(pos) => self.transitions[tr_index].priorities.insert(pos, over),
        }
    }

    /// Update all priorities to make a transitive closure
    ///
    /// # Errors
    /// `NetError::CyclicPriorities` is returned if there is a cyclic priority in the net
    pub fn update_priorities(&mut self) -> Result<(), Box<dyn Error>> {
        let mut done = IndexVec::<TransitionId, bool>::default();
        for tr in self.transitions.iter() {
            done.push(tr.priorities.is_empty());
        }

        if !done.iter().any(|&v| v) {
            return Err(Box::new(NetError::CyclicPriorities));
        }

        loop {
            if !done.iter().any(|&v| !v) {
                return Ok(());
            }
            let to_change: Vec<TransitionId> = done
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| if v { None } else { Some(TransitionId::from(i)) })
                .collect();
            let mut changed = false;
            for current_index in to_change {
                done[current_index] = self.transitions[current_index]
                    .priorities
                    .iter()
                    .all(|&i| done[i]);
                if done[current_index] {
                    changed = true;
                    let mut to_extend = vec![];
                    for &sub_index in &self.transitions[current_index].priorities {
                        to_extend.extend(&self.transitions[sub_index].priorities);
                    }
                    for e in to_extend {
                        self.add_priority(current_index, e);
                    }
                }
            }
            if !changed {
                return Err(Box::new(NetError::CyclicPriorities));
            }
        }
    }
}
