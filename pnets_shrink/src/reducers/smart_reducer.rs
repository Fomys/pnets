use std::collections::VecDeque;
use std::marker::PhantomData;

use crate::modifications::Modification;
use crate::reducers::reduce::{ConservativeReduce, PlaceReduce, TransitionReduce};
use crate::reducers::Reduce;

/// Execute a reduction, then on each transition and place modified we execute the actions
/// `PostPlace` and `PostTransition` on each of the modifications
pub struct SmartReducer<Net, Red, PostPlace, PostTransition>(
    PhantomData<(Net, Red, PostPlace, PostTransition)>,
)
where
    Red: Reduce<Net>,
    PostPlace: PlaceReduce<Net>,
    PostTransition: TransitionReduce<Net>;

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
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        let mut original_modifications = vec![];
        // Run original reduction
        Red::reduce(net, &mut original_modifications);
        // Fill a queue with all modification
        let mut modifications_queue = VecDeque::from(original_modifications);
        // While this queue is not empty, try to execute PostPlace and PostTransition
        while let Some(modification) = modifications_queue.pop_front() {
            match modification {
                Modification::Agglomeration(agg) => {
                    let mut modifications = vec![];
                    PostPlace::place_reduce(net, agg.new_place, &mut modifications);
                    modifications.push(Modification::Agglomeration(agg));
                    modifications_queue.extend(modifications);
                }
                other => modifications.push(other),
            }
        }
    }
}
