use pnets::standard::Net;
use pnets::PlaceId;

use crate::modifications::{InequalityReduction, Modification};
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce};
use crate::reducers::Reduce;

/// Removes the pattern "pl -> tr" from the network (pl and tr don't has other connection)
///
/// See definition 10, page 12 [STTT](https://doi.org/10.1007/s10009-019-00519-1)
pub struct SourceSinkReducer;

impl ConservativeReduce<Net> for SourceSinkReducer {}

impl Reduce<Net> for SourceSinkReducer {
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        for tr in (0..net.places.len()).map(|v| PlaceId::from(v)) {
            Self::place_reduce(net, tr, modifications);
        }
    }
}

impl PlaceReduce<Net> for SourceSinkReducer {
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
