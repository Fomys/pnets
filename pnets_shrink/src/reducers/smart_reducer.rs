use pnets::standard;
use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::modifications::Modification;
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce, TransitionReduce};
use crate::reducers::standard::{ParallelPlaceReducer, ParallelTransitionReducer};
use crate::reducers::Reduce;

/// Execute a reduction, then on each transition and place modified we execute the actions
/// `PostPlace` and `PostTransition` on each of the modifications
pub struct SmartReducer<Net, Red, PostPlace, PostTransition>
where
    Red: Reduce<Net>,
    PostPlace: PlaceReduce<Net>,
    PostTransition: TransitionReduce<Net>,
{
    reducer: Red,
    post_place: PostPlace,
    _post_transition: PostTransition,
    _ph: PhantomData<Net>,
}

impl<Net, Red, PostPlace, PostTransition> SmartReducer<Net, Red, PostPlace, PostTransition>
where
    Red: Reduce<Net>,
    PostPlace: PlaceReduce<Net>,
    PostTransition: TransitionReduce<Net>,
{
    /// Create a new smart reducer
    pub fn new(reducer: Red, post_place: PostPlace, post_transition: PostTransition) -> Self {
        Self {
            reducer,
            post_place,
            _post_transition: post_transition,
            _ph: Default::default(),
        }
    }
}
impl<Red> SmartReducer<standard::Net, Red, ParallelPlaceReducer, ParallelTransitionReducer>
where
    Red: Reduce<standard::Net>,
{
    /// Create a new identity smart reducer for identity place and transitions
    pub fn new_parallel_smart_reducer(reducer: Red) -> Self {
        Self::new(
            reducer,
            ParallelPlaceReducer::new(),
            ParallelTransitionReducer::new(),
        )
    }
}

impl<Net, Red, PostPlace, PostTransition> ConservativeReduce<Net>
    for SmartReducer<Net, Red, PostPlace, PostTransition>
where
    Red: ConservativeReduce<Net>,
    PostPlace: PlaceReduce<Net> + ConservativeReduce<Net>,
    PostTransition: TransitionReduce<Net> + ConservativeReduce<Net>,
{
}

impl<Net, Red, PostPlace, PostTransition> Reduce<Net>
    for SmartReducer<Net, Red, PostPlace, PostTransition>
where
    Red: Reduce<Net>,
    PostPlace: PlaceReduce<Net>,
    PostTransition: TransitionReduce<Net>,
{
    fn reduce(&self, net: &mut Net, modifications: &mut Vec<Modification>) {
        let mut original_modifications = vec![];
        // Run original reduction
        self.reducer.reduce(net, &mut original_modifications);
        // Fill a queue with all modification
        let mut modifications_queue = VecDeque::from(original_modifications);
        // While this queue is not empty, try to execute PostPlace and PostTransition
        while let Some(modification) = modifications_queue.pop_front() {
            match modification {
                Modification::Agglomeration(agg) => {
                    let mut new_modifications = vec![];
                    self.post_place
                        .place_reduce(net, agg.new_place, &mut new_modifications);
                    modifications.push(Modification::Agglomeration(agg));
                    modifications_queue.extend(new_modifications);
                }
                other => modifications.push(other),
            }
        }
    }
}
