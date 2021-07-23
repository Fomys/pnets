//! All modifications supported by this crate
//!
//! In this module you can found all modifications that are supported by this crate, you can found
//! detailed explanation in structures documentation.
use pnets::{PlaceId, TransitionId};

/// This modification correspond to an agglomeration of serval place in a unique new place.
///
/// The associated equation is [`Agglomeration::factor`] * [`Agglomeration::new_place`] =
/// [`Agglomeration::deleted_places`] + [`Agglomeration::constant`]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Agglomeration {
    /// Place which is created during the modification
    pub new_place: PlaceId,
    /// Factor for new place in the equation
    pub factor: isize,
    /// Places which are deleted during the modification
    ///
    /// This vector contains tuples ([`PlaceId`], [`isize`]) because sometimes you may need to add
    /// a weight on a specific place to keep the equation right.
    pub deleted_places: Vec<(PlaceId, isize)>,
    /// Constant to add at the end of equation
    pub constant: isize,
}

/// Deletion of several places because they are redundant with an other group of places.
///
/// The associated equation is [`Reduction::equals_to`] + [`Reduction::constant`] =
/// [`Reduction::deleted_places`]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reduction {
    /// Places which are kept during the modification
    ///
    /// This vector contains tuples ([`PlaceId`], [`isize`]) because sometimes you may need to add
    /// a weight on a specific place to keep the equation right.
    pub equals_to: Vec<(PlaceId, isize)>,
    /// Constant in the relation
    pub constant: isize,
    /// Places which are deleted during the modification
    ///
    /// This vector contains tuples ([`PlaceId`], [`isize`]) because sometimes you may need to add
    /// a weight on a specific place to keep the equation right.
    pub deleted_places: Vec<(PlaceId, isize)>,
}

/// Elimination of transitions
///
/// This reduction allows you to delete some transitions because they are redundant with others.
/// There is no equations associated with this reduction
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransitionElimination {
    /// Deleted transitions
    pub deleted_transitions: Vec<TransitionId>,
}

/// Inequality reduction
///
/// This modification deletes some places because they are redundant with a group of other places.
///
/// The equation associated is [`InequalityReduction::kept_places`] +
/// [`InequalityReduction::constant`] => [`InequalityReduction::deleted_places`]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InequalityReduction {
    /// Places which are kept during the modification
    ///
    /// This vector contains tuples ([`PlaceId`], [`isize`]) because sometimes you may need to add
    /// a weight on a specific place to keep the equation right.
    pub kept_places: Vec<(PlaceId, isize)>,
    /// Constant in the relation
    pub constant: isize,
    /// Places which are deleted during the modification
    ///
    /// This vector contains tuples ([`PlaceId`], [`isize`]) because sometimes you may need to add
    /// a weight on a specific place to keep the equation right.
    pub deleted_places: Vec<(PlaceId, isize)>,
}

/// All reductions supported by this library
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Modification {
    /// Agglomeration
    Agglomeration(Agglomeration),
    /// Reduction
    Reduction(Reduction),
    /// Transition elimination
    TransitionElimination(TransitionElimination),
    /// Inequality reduction
    InequalityReduction(InequalityReduction),
}
