use pnets::standard::Net;
use pnets::PlaceId;

use crate::modifications::{Modification, Reduction};
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce};
use crate::reducers::Reduce;

/// Removes identity places, which has a constant value
///
/// See Definition 2, page 5 [STTT](https://doi.org/10.1007/s10009-019-00519-1)
pub struct IdentityPlaceReducer;

impl IdentityPlaceReducer {
    /// New identity place reducer
    pub fn new() -> Self {
        Self {}
    }
}

impl ConservativeReduce<Net> for IdentityPlaceReducer {}

impl Reduce<Net> for IdentityPlaceReducer {
    fn reduce(&self, net: &mut Net, modifications: &mut Vec<Modification>) {
        for pl in (0..net.places.len()).map(|v| PlaceId::from(v)) {
            self.place_reduce(net, pl, modifications);
        }
    }
}

impl PlaceReduce<Net> for IdentityPlaceReducer {
    fn place_reduce(&self, net: &mut Net, pl: PlaceId, modifications: &mut Vec<Modification>) {
        // Search all places which has same consumers and producers
        if !net[pl].deleted
            && net[pl]
                .produced_by
                .iter_with(&net[pl].consumed_by)
                .all(|(_, w_produce, w_consume)| w_produce == w_consume)
        {
            modifications.push(Modification::Reduction(Reduction {
                deleted_places: vec![(pl, 1)],
                equals_to: vec![],
                constant: net[pl].initial as isize,
            }));
            net.delete_place(pl);
        }
    }
}
