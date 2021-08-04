#![forbid(missing_docs)]

//! # PNETS
//! Pnets is a framework for manipulating Petri nets
//!
//!
//! This crate provides an api for manipulating Petri nets.
//! Two main structures are provided by this library:
//! - [`standard::Net`] - which allows to manipulate classical Petri net;
//! - [`timed::Net`] - which allows the manipulation of temporal Petri net.
//!
//! In order to easily manipulate these nets this api provides the following elements:
//! - [`arc::Kind`] - an enum of the different types of arcs that exist in a Petri net;
//! - [`Marking`] - a structure for manipulating hollow vectors;
//! - [`PlaceId`] and [`TransitionId`] - a type for indexing places and transitions in networks.
//!
pub use errors::NetError;
pub use marking::Marking;
pub use net::{NodeId, PlaceId, TransitionId};

pub mod arc;
mod errors;
mod marking;
mod net;
pub mod standard;
pub mod timed;
