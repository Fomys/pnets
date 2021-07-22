use custom_derive::custom_derive;
use indexed_vec::Idx;
use newtype_derive::{
    newtype_as_item, newtype_fmt, newtype_wrap_bin_op, newtype_wrap_bin_op_assign, NewtypeAdd,
    NewtypeAddAssign, NewtypeDebug, NewtypeDisplay,
};

custom_derive! {
    /// Represent a transition identifier in the network
    #[derive(
         Ord, PartialOrd, Clone, Copy, Eq, PartialEq, Hash,
        NewtypeDebug, NewtypeDisplay, NewtypeAddAssign(usize), Default, NewtypeAdd(usize)
    )]
    pub struct TransitionId(usize);
}

impl Idx for TransitionId {
    fn new(v: usize) -> Self {
        Self::from(v)
    }

    fn index(self) -> usize {
        self.0
    }
}

custom_derive! {
    /// Represent a place identifier in the network
    #[derive(
        Ord, PartialOrd, Clone, Copy, Eq, PartialEq, Hash,
        NewtypeDebug, NewtypeDisplay, NewtypeAddAssign(usize), Default, NewtypeAdd(usize)
    )]
    pub struct PlaceId(usize);
}

impl Idx for PlaceId {
    fn new(v: usize) -> Self {
        Self::from(v)
    }

    fn index(self) -> usize {
        self.0
    }
}

impl ::std::convert::From<usize> for PlaceId {
    fn from(v: usize) -> Self {
        PlaceId(v)
    }
}

impl ::std::convert::From<usize> for TransitionId {
    fn from(v: usize) -> Self {
        TransitionId(v)
    }
}
