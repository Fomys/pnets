//! All reductions appliable to standard network
pub use identity_place_reducer::IdentityPlaceReducer;
pub use identity_transitions_reducer::IdentityTransitionReducer;
pub use invariant_reducer::InvariantReducer;
pub use parallel_place_reducer::ParallelPlaceReducer;
pub use parallel_transition_reducer::ParallelTransitionReducer;
pub use pseudo_start::PseudoStart;
pub use rl_reducer::RLReducer;
pub use simple_chain_agglomeration::SimpleChainReducer;
pub use simple_loop_agglomeration::SimpleLoopAgglomeration;
pub use source_sink_reducer::SourceSinkReducer;
pub use weight_simplification::WeightSimplification;

use crate::reducers::SmartReducer;

mod identity_place_reducer;
mod identity_transitions_reducer;
mod invariant_reducer;
mod parallel_place_reducer;
mod parallel_transition_reducer;
mod pseudo_start;
mod rl_reducer;
mod simple_chain_agglomeration;
mod simple_loop_agglomeration;
mod source_sink_reducer;
mod weight_simplification;

/// Alias to Smart reducer which try to remove parallel places and parallel transitions after each modifications
pub type ParallelSmartReducer<Net, Red> =
    SmartReducer<Net, Red, ParallelPlaceReducer, ParallelTransitionReducer>;
