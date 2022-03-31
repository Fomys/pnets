use pnets::standard::Net;
use pnets::{arc, PlaceId, TransitionId};

use crate::modifications::{Agglomeration, Modification};
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce};
use crate::reducers::Reduce;

/// Replaces a place that has an initial marking equal to all the weights of the related arcs
pub struct WeightSimplification;

impl WeightSimplification {
    /// Create a new Weight simplification reducer
    pub fn new() -> Self {
        Self {}
    }
}

impl ConservativeReduce<Net> for WeightSimplification {}

impl Reduce<Net> for WeightSimplification {
    fn reduce(&self, net: &mut Net, modifications: &mut Vec<Modification>) {
        for tr in (0..net.places.len()).map(|v| PlaceId::from(v)) {
            self.place_reduce(net, tr, modifications);
        }
    }
}

impl PlaceReduce<Net> for WeightSimplification {
    fn place_reduce(&self, net: &mut Net, pl: PlaceId, modifications: &mut Vec<Modification>) {
        // Check if the place have an initial marking greater than 1 and all its arc with the weight
        // of its initial marking
        if !net[pl].deleted
            && net[pl].initial > 1
            && net[pl]
                .produced_by
                .iter()
                .all(|(_, w)| *w == net[pl].initial)
            && net[pl]
                .consumed_by
                .iter()
                .all(|(_, w)| *w == net[pl].initial)
        {
            // Create the new place and copy all arcs
            let new_pl = net.create_place();
            net[new_pl].initial = 1;
            let consume_transitions: Vec<TransitionId> =
                net[pl].consumed_by.iter().map(|(tr, _)| *tr).collect();
            for tr in consume_transitions {
                net.add_arc(arc::Kind::Consume(new_pl, tr, 1)).unwrap();
            }
            let produce_transitions: Vec<TransitionId> =
                net[pl].produced_by.iter().map(|(tr, _)| *tr).collect();
            for tr in produce_transitions {
                net.add_arc(arc::Kind::Produce(new_pl, tr, 1)).unwrap();
            }
            net.delete_place(pl);
            modifications.push(Modification::Agglomeration(Agglomeration {
                deleted_places: vec![(pl, 1)],
                new_place: new_pl,
                constant: 0,
                factor: net[pl].initial as isize,
            }));
        }
    }
}
