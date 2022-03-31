use std::marker::PhantomData;

use pnets::{PlaceId, TransitionId};

use crate::modifications::Modification;
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce, TransitionReduce};
use crate::reducers::{IdentityReducer, Reduce};

/// Apply the first reduction and then the second reduction
///
/// This struct also implement [`PlaceReduce`] and [`TransitionReduce`] to allows chaining
/// reductions on a specific place or transition
#[derive(Default)]
pub struct ChainReducer<Net, First, Second>
where
    First: Reduce<Net>,
    Second: Reduce<Net>,
{
    first: First,
    second: Second,
    _ph: PhantomData<Net>,
}

impl<Net> ChainReducer<Net, IdentityReducer, IdentityReducer> {
    /// Create a new chain reducer with two reductions
    pub fn new2<First: Reduce<Net>, Second: Reduce<Net>>(
        first: First,
        second: Second,
    ) -> ChainReducer<Net, First, Second> {
        ChainReducer {
            first,
            second,
            _ph: PhantomData::default(),
        }
    }
    /// Create a new chain reducer with three reductions
    pub fn new3<First: Reduce<Net>, Second: Reduce<Net>, Third: Reduce<Net>>(
        first: First,
        second: Second,
        third: Third,
    ) -> ChainReducer<Net, First, ChainReducer<Net, Second, Third>> {
        Self::new2(first, Self::new2(second, third))
    }

    /// Create a new chain reducer with four reductions
    pub fn new4<
        First: Reduce<Net>,
        Second: Reduce<Net>,
        Third: Reduce<Net>,
        Fourth: Reduce<Net>,
    >(
        first: First,
        second: Second,
        third: Third,
        fourth: Fourth,
    ) -> ChainReducer<Net, First, ChainReducer<Net, Second, ChainReducer<Net, Third, Fourth>>> {
        Self::new3(first, second, Self::new2(third, fourth))
    }

    /// Create a new chain reducer with five reductions
    pub fn new5<
        First: Reduce<Net>,
        Second: Reduce<Net>,
        Third: Reduce<Net>,
        Fourth: Reduce<Net>,
        Fifth: Reduce<Net>,
    >(
        first: First,
        second: Second,
        third: Third,
        fourth: Fourth,
        fifth: Fifth,
    ) -> ChainReducer<
        Net,
        First,
        ChainReducer<Net, Second, ChainReducer<Net, Third, ChainReducer<Net, Fourth, Fifth>>>,
    > {
        Self::new4(first, second, third, Self::new2(fourth, fifth))
    }

    /// Create a new chain reducer with six reductions
    pub fn new6<
        First: Reduce<Net>,
        Second: Reduce<Net>,
        Third: Reduce<Net>,
        Fourth: Reduce<Net>,
        Fifth: Reduce<Net>,
        Sixth: Reduce<Net>,
    >(
        first: First,
        second: Second,
        third: Third,
        fourth: Fourth,
        fifth: Fifth,
        sixth: Sixth,
    ) -> ChainReducer<
        Net,
        First,
        ChainReducer<
            Net,
            Second,
            ChainReducer<Net, Third, ChainReducer<Net, Fourth, ChainReducer<Net, Fifth, Sixth>>>,
        >,
    > {
        Self::new5(first, second, third, fourth, Self::new2(fifth, sixth))
    }
}

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
    fn reduce(&self, net: &mut Net, modifications: &mut Vec<Modification>) {
        self.first.reduce(net, modifications);
        self.second.reduce(net, modifications);
    }
}

impl<Net, First, Second> PlaceReduce<Net> for ChainReducer<Net, First, Second>
where
    First: PlaceReduce<Net>,
    Second: PlaceReduce<Net>,
{
    fn place_reduce(&self, net: &mut Net, pl: PlaceId, modifications: &mut Vec<Modification>) {
        self.first.place_reduce(net, pl, modifications);
        self.second.place_reduce(net, pl, modifications)
    }
}

impl<Net, First, Second> TransitionReduce<Net> for ChainReducer<Net, First, Second>
where
    First: TransitionReduce<Net>,
    Second: TransitionReduce<Net>,
{
    fn transition_reduce(
        &self,
        net: &mut Net,
        tr: TransitionId,
        modifications: &mut Vec<Modification>,
    ) {
        self.first.transition_reduce(net, tr, modifications);
        self.second.transition_reduce(net, tr, modifications)
    }
}
