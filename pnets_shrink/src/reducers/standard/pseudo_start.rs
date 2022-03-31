use pnets::standard::Net;
use pnets::PlaceId;

use crate::modifications::{Agglomeration, Modification, Reduction};
use crate::reducers::reduce::PlaceReduce;
use crate::reducers::Reduce;

/// Reduction for Election2020 net
pub struct PseudoStart;

impl PseudoStart {
    /// Create a new pseudo start reducer
    pub fn new() -> Self {
        Self {}
    }
}

impl Reduce<Net> for PseudoStart {
    fn reduce(&self, net: &mut Net, modifications: &mut Vec<Modification>) {
        for pl in (0..net.places.len()).map(|pl| PlaceId::from(pl)) {
            self.place_reduce(net, pl, modifications);
        }
    }
}

impl PlaceReduce<Net> for PseudoStart {
    fn place_reduce(&self, net: &mut Net, pl: PlaceId, modifications: &mut Vec<Modification>) {
        if !net[pl].deleted
            && net[pl].initial == 1
            && net[pl].produced_by.is_empty()
            && net[pl].consumed_by.iter().all(|(tr, w)| {
                *w == 1
                    && net[*tr].consume.len() == 1
                    && net[*tr].produce.len() == 1
                    && net[net[*tr].produce.iter().next().unwrap().0]
                        .consumed_by
                        .is_empty()
            })
        {
            let mut new_places = vec![];
            let mut tmp_places = vec![];
            let consumed_by: Vec<(PlaceId, usize)> = net[pl]
                .consumed_by
                .iter()
                .map(|(tr, _)| *net[*tr].produce.iter().next().unwrap())
                .collect();

            for (pl, w) in consumed_by {
                let new_pl = net.clone_place(pl);
                let tmp = net.create_place();
                net.delete_place(tmp);
                net.delete_place(pl);

                tmp_places.push(tmp);
                new_places.push(new_pl);
                modifications.push(Modification::Agglomeration(Agglomeration {
                    deleted_places: vec![(pl, 1), (tmp, -(w as isize))],
                    constant: 0,
                    new_place: new_pl,
                    factor: 1,
                }));
            }
            modifications.push(Modification::Reduction(Reduction {
                deleted_places: tmp_places
                    .iter()
                    .map(|pl| (*pl, 1))
                    .chain(vec![(pl, 1)])
                    .collect(),
                equals_to: vec![],
                constant: net.places[pl].initial as isize,
            }));

            // Suppression de la place source
            while let Some(&(tr, _)) = net[pl].consumed_by.iter().next() {
                net.delete_transition(tr);
            }

            while let Some(&(tr, _)) = net[pl].produced_by.iter().next() {
                net.delete_transition(tr);
            }
            net.delete_place(pl);
        }
    }
}
