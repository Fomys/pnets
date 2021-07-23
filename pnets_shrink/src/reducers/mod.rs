//! All reducers supported by this crate
//!
//! This module contains an API to create reductions and some already implemented reductions.
pub use chain_reducer::*;
pub use identity_reducer::IdentityReducer;
pub use loop_reducer::LoopReducer;
pub use reduce::{ConservativeReduce, PlaceReduce, Reduce, TransitionReduce};
pub use smart_reducer::SmartReducer;

mod chain_reducer;
mod identity_reducer;
mod loop_reducer;
mod reduce;
mod smart_reducer;
pub mod standard;
