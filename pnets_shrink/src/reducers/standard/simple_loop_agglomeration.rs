use indexed_vec::IndexVec;

use pnets::standard::Net;
use pnets::{arc, PlaceId, TransitionId};

use crate::modifications::{Agglomeration, Modification};
use crate::reducers::reduce::ConservativeReduce;
use crate::reducers::Reduce;

#[derive(Clone)]
struct SimpleLoopAgglomerationGraphNode {
    connected: Vec<(TransitionId, PlaceId)>,
    num: Option<usize>,
    accessible_num: Option<usize>,
    in_stack: bool,
}

#[derive(Default)]
struct Partition {
    transitions: Vec<TransitionId>,
    places: Vec<PlaceId>,
}

impl Partition {
    pub fn add(&mut self, tr: TransitionId, pl: PlaceId) {
        match self.transitions.binary_search(&tr) {
            Ok(_) => {} // element already in vector @ `pos`
            Err(pos) => self.transitions.insert(pos, tr),
        }
        match self.places.binary_search(&pl) {
            Ok(_) => {} // element already in vector @ `pos`
            Err(pos) => self.places.insert(pos, pl),
        }
    }
}

/// Implementation of the Tarjan algorithm on petri network
/// 
/// We apply this algorithm on the graph formed by all transition which has only one consumption and
/// one production (those transitions correspond to those found in the SLA reduction)
struct SimpleLoopAgglomerationGraph {
    pub nodes: IndexVec<TransitionId, SimpleLoopAgglomerationGraphNode>,
    partitions: Vec<Partition>,
    current_num: usize,
    stack: Vec<(TransitionId, PlaceId)>,
}

impl SimpleLoopAgglomerationGraph {
    pub fn new(node_count: usize) -> Self {
        let nodes: IndexVec<TransitionId, SimpleLoopAgglomerationGraphNode> = IndexVec::from_elem_n(
            SimpleLoopAgglomerationGraphNode {
                connected: vec![],
                num: None,
                accessible_num: None,
                in_stack: false,
            },
            node_count,
        );
        Self {
            nodes,
            partitions: vec![],
            current_num: 0,
            stack: vec![],
        }
    }

    fn walk(&mut self, net: &Net, source_tr: TransitionId) {
        let source_transition = &net[source_tr];

        // We know that source_tr has only one input/output place, with a weight of 1
        if (source_transition.consume.len() != 1)
            || source_transition.produce.len() != 1
            || (source_transition.consume.iter().next().unwrap().1 != 1)
            || (source_transition.produce.iter().next().unwrap().1 != 1)
        {
            return;
        }
        self.nodes[source_tr].num = Some(self.current_num);
        self.nodes[source_tr].accessible_num = Some(self.current_num);
        self.nodes[source_tr].in_stack = true;
        self.current_num += 1;

        let pl = match source_transition.produce.iter().next() {
            Some(&(pl, _)) => pl,
            _ => return,
        };
        let place = &net[pl];
        self.stack.push((source_tr, pl));
        for &tr in place.consumed_by.iter().filter_map(|(tr, _)| {
            if net[*tr].consume.len() == 1
                && net[*tr].produce.len() == 1
                && (source_transition.consume.iter().next().unwrap().1 == 1)
                && (source_transition.produce.iter().next().unwrap().1 == 1)
            {
                Some(tr)
            } else {
                None
            }
        }) {
            if self.nodes[tr].num.is_none() {
                self.walk(net, tr);
                self.nodes[source_tr].accessible_num = Some(
                    self.nodes[source_tr]
                        .accessible_num
                        .unwrap()
                        .min(self.nodes[tr].accessible_num.unwrap()),
                );
            } else if self.nodes[tr].in_stack {
                self.nodes[source_tr].accessible_num = Some(
                    self.nodes[source_tr]
                        .accessible_num
                        .unwrap()
                        .min(self.nodes[tr].num.unwrap()),
                );
            }
        }

        if self.nodes[source_tr].accessible_num == self.nodes[source_tr].num {
            let mut component = Partition::default();
            while let Some((tr, pl)) = self.stack.pop() {
                component.add(tr, pl);
                if tr == source_tr {
                    break;
                }
            }
            if component.transitions.len() > 1 && component.places.len() > 1 {
                self.partitions.push(component);
            }
        }
    }

    pub fn compute_partitions(&mut self, net: &Net) {
        for n in (0..self.nodes.len()).map(|n| TransitionId::from(n)) {
            if self.nodes[n].num.is_none() {
                self.walk(net, n);
            }
        }
    }
}

/// Remove simple loopd from the network and replace them by a unique place
///
/// See Definition 6, page 8 [STTT](https://doi.org/10.1007/s10009-019-00519-1)
pub struct SimpleLoopAgglomeration;

impl ConservativeReduce<Net> for SimpleLoopAgglomeration {}

impl Reduce<Net> for SimpleLoopAgglomeration {
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        // Compute partitions of the transition graph
        let mut graph = SimpleLoopAgglomerationGraph::new(net.transitions.len());
        graph.compute_partitions(net);

        // Each partition can be replaced with a new agglomerated place
        for partition in &graph.partitions {
            let new_place = net.create_place();
            // Copy all arcs and merge initial marking in the new place
            let mut arcs = vec![];
            let mut initial = 0;
            for &pl in &partition.places {
                arcs.extend(net[pl].get_arcs());
                initial += net[pl].initial;
                net.delete_place(pl);
            }
            net[new_place].initial = initial;
            for original_arc in &arcs {
                match *original_arc {
                    arc::Kind::Consume(_, tr, w) => {
                        net.add_arc(arc::Kind::Consume(new_place, tr, w)).unwrap();
                    }
                    arc::Kind::Produce(_, tr, w) => {
                        net.add_arc(arc::Kind::Produce(new_place, tr, w)).unwrap();
                    }
                    _ => {}
                }
            }
            modifications.push(Modification::Agglomeration(Agglomeration {
                deleted_places: partition.places.iter().map(|pl| (*pl, 1)).collect(),
                new_place,
                constant: 0,
                factor: 1,
            }));
        }
    }
}
