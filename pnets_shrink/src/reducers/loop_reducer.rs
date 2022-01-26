use std::env;
use std::marker::PhantomData;

use crate::modifications::Modification;
use crate::reducers::reduce::ConservativeReduce;
use crate::reducers::Reduce;

/// Executes a reduction rule in a loop as long as there have been changes in the net
pub struct LoopReducer<Net, Red>(PhantomData<(Red, Net)>)
where
    Red: Reduce<Net>;

impl<Net, Red> ConservativeReduce<Net> for LoopReducer<Net, Red> where Red: ConservativeReduce<Net> {}

impl<Net, Red> Reduce<Net> for LoopReducer<Net, Red>
where
    Red: Reduce<Net>,
{
    fn reduce(net: &mut Net, modifications: &mut Vec<Modification>) {
        let mut max_iter: u64 = env::var("MAX_ITER")
            .unwrap_or("a".to_string())
            .parse()
            .unwrap_or(u64::MAX);
        while max_iter > 0 {
            max_iter -= 1;
            let original_count = modifications.len();
            Red::reduce(net, modifications);
            if modifications.len() == original_count {
                break;
            }
        }
    }
}
