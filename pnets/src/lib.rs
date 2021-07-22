#![forbid(missing_docs)]

//! # PNETS
//! Pnets is a framework for manipulating petri networks
//!
//!
//! This crate provides an api for manipulating petri nets.
//! Two main structures are provided by this library:
//! - [`standard::Net`] - which allows to manipulate classical petri nets;
//! - [`timed::Net`] - which allows the manipulation of temporal petri nets.
//!
//! In order to easily manipulate these nets this api provides the following elements:
//! - [`arc::Kind`] - an enum of the different types of arcs that exist in a petri net;
//! - [`Marking`] - a structure for manipulating hollow vectors;
//! - [`PlaceId`] and [`TransitionId`] - a type for indexing places and transitions in networks.
//!
pub use errors::NetError;
pub use marking::Marking;
pub use net::{PlaceId, TransitionId};

pub mod arc;
mod errors;
pub mod io;
mod marking;
mod net;
pub mod standard;
pub mod timed;
