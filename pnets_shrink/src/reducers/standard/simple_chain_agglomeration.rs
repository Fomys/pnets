use pnets::standard::Net;
use pnets::{arc, TransitionId};

use crate::modifications::{Agglomeration, Modification};
use crate::reducers::reduce::{ConservativeReduce, TransitionReduce};
use crate::reducers::Reduce;

/// Remove simple chains from the net and replace them by a unique place
///
/// See Definition 5, page 8 [STTT](https://doi.org/10.1007/s10009-019-00519-1)
pub struct SimpleChainReducer;

impl SimpleChainReducer {
    /// Create a new simple chain reducer
    pub fn new() -> Self {
        Self {}
    }
}

impl ConservativeReduce<Net> for SimpleChainReducer {}

impl Reduce<Net> for SimpleChainReducer {
    fn reduce(&self, net: &mut Net, modifications: &mut Vec<Modification>) {
        if !net.transitions.is_empty() {
            for tr in (0..net.transitions.len()).map(|v| TransitionId::from(v)) {
                self.transition_reduce(net, tr, modifications);
            }
        }
    }
}

impl TransitionReduce<Net> for SimpleChainReducer {
    fn transition_reduce(
        &self,
        net: &mut Net,
        tr: TransitionId,
        modifications: &mut Vec<Modification>,
    ) {
        // We search simple chain agglomeration by searching the middle transition, which has only
        // one consumption and one production.
        if !net[tr].deleted && net[tr].produce.len() == 1 && net[tr].consume.len() == 1 {
            let pl_dest = net[tr].produce.iter().next().unwrap().0;
            // Check if the output transition has only one producer and an empty initial marking
            if !net[pl_dest].deleted
                && net[pl_dest].produced_by.len() == 1
                && net[pl_dest].initial == 0
            {
                let pl_source = net[tr].consume.iter().next().unwrap().0;
                let new_pl = net.create_place();

                // Copy old arcs to the new agglomerated place
                for old_arc in net[pl_source]
                    .get_arcs()
                    .iter()
                    .chain(net[pl_dest].get_arcs().iter())
                {
                    match *old_arc {
                        arc::Kind::Consume(_, tr, w) => {
                            net.add_arc(arc::Kind::Consume(new_pl, tr, w)).unwrap();
                        }
                        arc::Kind::Produce(_, tr, w) => {
                            net.add_arc(arc::Kind::Produce(new_pl, tr, w)).unwrap();
                        }
                        _ => {}
                    }
                }
                net[new_pl].initial = net[pl_source].initial;

                // Delete old places and transition
                net.delete_place(pl_dest);
                net.delete_place(pl_source);
                net.delete_transition(tr);

                modifications.push(Modification::Agglomeration(Agglomeration {
                    deleted_places: vec![(pl_source, 1), (pl_dest, 1)],
                    new_place: new_pl,
                    constant: 0,
                    factor: 1,
                }));
            }
        }
    }
}
