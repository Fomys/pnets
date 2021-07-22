//! Standard Petri net
//!
//! Represent a Petri net with only places, transitions, consume and produce arcs.

pub use net::Net;
pub use place::Place;
pub use transition::Transition;

mod net;
mod place;
mod transition;
