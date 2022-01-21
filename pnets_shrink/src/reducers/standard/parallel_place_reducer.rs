use pnets::standard::Net;
use pnets::PlaceId;

use crate::modifications::{Modification, Reduction};
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce};
use crate::reducers::Reduce;

/// Removes redundant places which have the same producers and consumers
///
/// See Definition 2, page 5 [STTT](https://doi.org/10.1007/s10009-019-00519-1)
pub struct ParallelPlaceReducer;

impl ConservativeReduce<Net> for ParallelPlaceReducer {}

impl Reduce<Net> for ParallelPlaceReducer {
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        for pl in (0..net.places.len()).map(|pl| PlaceId::from(pl)) {
            Self::place_reduce(net, pl, modifications);
        }
    }
}

impl PlaceReduce<Net> for ParallelPlaceReducer {
    fn place_reduce(net: &mut Net, pl_1: PlaceId, modifications: &mut Vec<Modification>) {
        if !net[pl_1].deleted {
            let mut places_to_delete = vec![];
            // Iterate over all connected transition for this place
            for pl_2 in net[pl_1]
                .produced_by
                .iter()
                .flat_map(|&(tr, _)| net[tr].produce.iter())
                .chain(
                    net[pl_1]
                        .consumed_by
                        .iter()
                        .flat_map(|&(tr, _)| net[tr].consume.iter()),
                )
                .map(|(pl, _)| *pl)
                .filter(|&pl_2| {
                    // Search places which are equals to pl_1
                    pl_1 != pl_2
                        && net[pl_1].consumed_by.len() == net[pl_2].consumed_by.len()
                        && net[pl_1].produced_by.len() == net[pl_2].produced_by.len()
                        && net[pl_1]
                            .produced_by
                            .iter_with(&net[pl_2].produced_by)
                            .all(|(_, w_1, w_2)| w_1 == w_2)
                        && net[pl_1]
                            .consumed_by
                            .iter_with(&net[pl_2].consumed_by)
                            .all(|(_, w_1, w_2)| w_1 == w_2)
                })
            {
                if Some(&pl_2) != places_to_delete.last() {
                    modifications.push(Modification::Reduction(Reduction {
                        deleted_places: vec![(pl_2, 1)],
                        equals_to: vec![(pl_1, 1)],
                        constant: 0,
                    }));
                    places_to_delete.push(pl_2);
                }
            }
            for pl in places_to_delete {
                net.delete_place(pl);
            }
        }
    }
}
