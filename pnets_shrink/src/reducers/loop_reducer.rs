use std::marker::PhantomData;

use crate::modifications::Modification;
use crate::reducers::reduce::ConservativeReduce;
use crate::reducers::Reduce;

/// Executes a reduction rule in a loop as long as there have been changes in the network
pub struct LoopReducer<Net, Red>(PhantomData<(Red, Net)>)
where
    Red: Reduce<Net>;

impl<Net, Red> ConservativeReduce<Net> for LoopReducer<Net, Red> where Red: ConservativeReduce<Net> {}

impl<Net, Red> Reduce<Net> for LoopReducer<Net, Red>
where
    Red: Reduce<Net>,
{
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        loop {
            let original_count = modifications.len();
            Red::reduce(net, modifications);
            if modifications.len() == original_count {
                break;
            }
        }
    }
}
