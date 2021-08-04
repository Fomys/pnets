use pnets::standard::Net;
use pnets::PlaceId;

use crate::modifications::{InequalityReduction, Modification};
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce};
use crate::reducers::Reduce;

/// Removes the pattern "pl -> tr" from the network (pl and tr don't has other connection)
pub struct R7Reducer;

impl ConservativeReduce<Net> for R7Reducer {}

impl Reduce<Net> for R7Reducer {
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        for tr in (0..net.places.len()).map(|v| PlaceId::from(v)) {
            Self::place_reduce(net, tr, modifications);
        }
    }
}

impl PlaceReduce<Net> for R7Reducer {
    fn place_reduce(net: &mut Net, pl: PlaceId, modifications: &mut Vec<Modification>) {
        // We check that the place is connected to only one transition
        if !net[pl].deleted && net[pl].produced_by.is_empty() && net[pl].consumed_by.len() == 1 {
            let &(tr, _) = net[pl].consumed_by.iter().next().unwrap();
            if net[tr].produce.is_empty() {
                net.delete_place(pl);
                modifications.push(Modification::InequalityReduction(InequalityReduction {
                    deleted_places: vec![(pl, 1)],
                    kept_places: vec![],
                    constant: net[pl].initial as isize,
                }));
            }
        }
    }
}
