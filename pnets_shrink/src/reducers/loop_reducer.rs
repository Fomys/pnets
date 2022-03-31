use std::marker::PhantomData;

use crate::modifications::Modification;
use crate::reducers::reduce::ConservativeReduce;
use crate::reducers::Reduce;

/// Executes a reduction rule in a loop as long as there have been changes in the net
pub struct LoopReducer<Net, Red>
where
    Red: Reduce<Net>,
{
    reducer: Red,
    max_iter: u64,
    _ph: PhantomData<Net>,
}

impl<Net, Red> LoopReducer<Net, Red>
where
    Red: Reduce<Net>,
{
    /// Create a new loop reducer
    ///
    /// reducer: reducer to loop over
    /// max_iter: maximum iteration count (useful if there is a lot of consecutive reductions)
    pub fn new(reducer: Red, max_iter: u64) -> Self {
        Self {
            reducer,
            max_iter,
            _ph: PhantomData::default(),
        }
    }
}

impl<Net, Red> ConservativeReduce<Net> for LoopReducer<Net, Red> where Red: ConservativeReduce<Net> {}

impl<Net, Red> Reduce<Net> for LoopReducer<Net, Red>
where
    Red: Reduce<Net>,
{
    fn reduce(&self, net: &mut Net, modifications: &mut Vec<Modification>) {
        if self.max_iter == u64::MAX {
            loop {
                let original_count = modifications.len();
                self.reducer.reduce(net, modifications);
                if modifications.len() == original_count {
                    break;
                }
            }
        } else {
            let mut iter = self.max_iter;
            while iter > 0 {
                iter -= 1;
                let original_count = modifications.len();
                self.reducer.reduce(net, modifications);
                if modifications.len() == original_count {
                    break;
                }
            }
        }
    }
}
