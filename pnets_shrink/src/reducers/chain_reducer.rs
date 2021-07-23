use std::marker::PhantomData;

use pnets::{PlaceId, TransitionId};

use crate::modifications::Modification;
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce, TransitionReduce};
use crate::reducers::Reduce;

/// Chain 3 reducers, see [`ChainReducer`]
pub type Chain3Reducer<Net, First, Second, Third> =
    ChainReducer<Net, First, ChainReducer<Net, Second, Third>>;
/// Chain 4 reducers, see [`ChainReducer`]
pub type Chain4Reducer<Net, First, Second, Third, Fourth> =
    Chain3Reducer<Net, First, Second, ChainReducer<Net, Third, Fourth>>;
/// Chain 5 reducers, see [`ChainReducer`]
pub type Chain5Reducer<Net, First, Second, Third, Fourth, Fifth> =
    Chain4Reducer<Net, First, Second, Third, ChainReducer<Net, Fourth, Fifth>>;
/// Chain 6 reducers, see [`ChainReducer`]
pub type Chain6Reducer<Net, First, Second, Third, Fourth, Fifth, Sixth> =
    Chain5Reducer<Net, First, Second, Third, Fourth, ChainReducer<Net, Fifth, Sixth>>;

/// Apply the first reduction and then the second reduction
///
/// This struct also implement [`PlaceReduce`] and [`TransitionReduce`] to allows chaining
/// reductions on a specific place or transition
#[derive(Default)]
pub struct ChainReducer<Net, First, Second>(PhantomData<(First, Second, Net)>)
where
    First: Reduce<Net>,
    Second: Reduce<Net>;

impl<Net, First, Second> ConservativeReduce<Net> for ChainReducer<Net, First, Second>
where
    First: ConservativeReduce<Net>,
    Second: ConservativeReduce<Net>,
{
}

impl<Net, First, Second> Reduce<Net> for ChainReducer<Net, First, Second>
where
    First: Reduce<Net>,
    Second: Reduce<Net>,
{
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        First::reduce(net, modifications);
        Second::reduce(net, modifications);
    }
}

impl<Net, First, Second> PlaceReduce<Net> for ChainReducer<Net, First, Second>
where
    First: PlaceReduce<Net>,
    Second: PlaceReduce<Net>,
{
    fn place_reduce(net: &mut Net, pl: PlaceId, modifications: &mut Vec<Modification>) {
        First::place_reduce(net, pl, modifications);
        Second::place_reduce(net, pl, modifications)
    }
}

impl<Net, First, Second> TransitionReduce<Net> for ChainReducer<Net, First, Second>
where
    First: TransitionReduce<Net>,
    Second: TransitionReduce<Net>,
{
    fn transition_reduce(net: &mut Net, tr: TransitionId, modifications: &mut Vec<Modification>) {
        First::transition_reduce(net, tr, modifications);
        Second::transition_reduce(net, tr, modifications)
    }
}
