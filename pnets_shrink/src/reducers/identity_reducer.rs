use pnets::{PlaceId, TransitionId};

use crate::modifications::Modification;
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce, TransitionReduce};
use crate::reducers::Reduce;

/// Identity reducer does nothing on the network
///
/// This reduction exists only to be able to easily test reduction chains or replace required parameters in others.
pub struct IdentityReducer {}

impl<Net> Reduce<Net> for IdentityReducer {
    fn reduce(_: &mut Net, _: &mut Vec<Modification>) {}
}

impl<Net> ConservativeReduce<Net> for IdentityReducer {}

impl<Net> PlaceReduce<Net> for IdentityReducer {
    fn place_reduce(_: &mut Net, _: PlaceId, _: &mut Vec<Modification>) {}
}

impl<Net> TransitionReduce<Net> for IdentityReducer {
    fn transition_reduce(_: &mut Net, _: TransitionId, _: &mut Vec<Modification>) {}
}
