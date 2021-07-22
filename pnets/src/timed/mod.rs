//! Network with time support
//!
//! Represent a petri network with time support and extra arc kind such as [`arc::Kind::Test`][`crate::arc::Kind::Test`].

pub use net::Net;
pub use place::Place;
pub use time_range::{Bound, TimeRange};
pub use transition::Transition;

mod net;
mod place;
pub mod time_range;
mod transition;
