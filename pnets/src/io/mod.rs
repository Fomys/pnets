//! All traits related to parsing and writing of a network
//!
//! They are useful to create new parsers and writer which should be compatible with this framework

pub use export::Export;
pub use parse::Parse;

mod export;
mod parse;
