use pnets::standard::Net;
use pnets::TransitionId;

use crate::modifications::{Modification, TransitionElimination};
use crate::reducers::reduce::{ConservativeReduce, TransitionReduce};
use crate::reducers::Reduce;

/// Removes identity transitions, which doesn't have an effect on the network
///
/// See Definition 1, page 5 [STTT](https://doi.org/10.1007/s10009-019-00519-1)
pub struct IdentityTransitionReducer;

impl ConservativeReduce<Net> for IdentityTransitionReducer {}

impl Reduce<Net> for IdentityTransitionReducer {
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        if !net.transitions.is_empty() {
            for tr in (0..net.transitions.len()).map(|v| TransitionId::from(v)) {
                Self::transition_reduce(net, tr, modifications);
            }
        }
    }
}

impl TransitionReduce<Net> for IdentityTransitionReducer {
    fn transition_reduce(net: &mut Net, tr: TransitionId, modifications: &mut Vec<Modification>) {
        // Search all transition which has the same consumption and the same production
        if !net[tr].is_disconnected()
            && net[tr]
                .produce
                .iter_with(&net[tr].consume)
                .all(|(_, w_produce, w_consume)| w_produce == w_consume)
        {
            modifications.push(Modification::TransitionElimination(TransitionElimination {
                deleted_transitions: vec![net[tr].id()],
            }));
            net.delete_transition(tr);
        }
    }
}
