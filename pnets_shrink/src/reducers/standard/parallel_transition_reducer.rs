use pnets::standard::Net;
use pnets::TransitionId;

use crate::modifications::{Modification, TransitionElimination};
use crate::reducers::reduce::{ConservativeReduce, TransitionReduce};
use crate::reducers::Reduce;

/// Removes redundant transitions which have the same producers and consumers
///
/// See Definition 1, page 5 [STTT](https://doi.org/10.1007/s10009-019-00519-1)
pub struct ParallelTransitionReducer;

impl ParallelTransitionReducer {
    /// Create a new parallel transition reducer
    pub fn new() -> Self {
        Self {}
    }
}

impl ConservativeReduce<Net> for ParallelTransitionReducer {}

impl Reduce<Net> for ParallelTransitionReducer {
    fn reduce(&self, net: &mut Net, modifications: &mut Vec<Modification>) {
        for tr in (0..net.transitions.len()).map(|tr| TransitionId::from(tr)) {
            self.transition_reduce(net, tr, modifications);
        }
    }
}

impl TransitionReduce<Net> for ParallelTransitionReducer {
    fn transition_reduce(
        &self,
        net: &mut Net,
        tr_1: TransitionId,
        modifications: &mut Vec<Modification>,
    ) {
        if !net[tr_1].deleted {
            let mut transitions_to_delete = vec![];
            // Iterate over all connected transition for this transition
            for tr_2 in net[tr_1]
                .produce
                .iter()
                .flat_map(|&(pl, _)| net[pl].produced_by.iter())
                .chain(
                    net[tr_1]
                        .consume
                        .iter()
                        .flat_map(|&(pl, _)| net[pl].produced_by.iter()),
                )
                .map(|(tr, _)| *tr)
                .filter(|&tr_2| {
                    // Search places which are equals to pl_1
                    tr_1 != tr_2
                        && net[tr_1].consume.len() == net[tr_2].consume.len()
                        && net[tr_1].produce.len() == net[tr_2].produce.len()
                        && net[tr_1]
                            .produce
                            .iter_with(&net[tr_2].produce)
                            .all(|(_, w_1, w_2)| w_1 == w_2)
                        && net[tr_1]
                            .consume
                            .iter_with(&net[tr_2].consume)
                            .all(|(_, w_1, w_2)| w_1 == w_2)
                })
            {
                modifications.push(Modification::TransitionElimination(TransitionElimination {
                    deleted_transitions: vec![tr_2],
                }));
                transitions_to_delete.push(tr_2);
            }
            for transition in transitions_to_delete {
                net.delete_transition(transition);
            }
        }
    }
}
