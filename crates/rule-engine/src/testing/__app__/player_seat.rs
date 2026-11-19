use derive_more::{Deref, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, Display)]
#[display("P{_0}")]
pub struct PlayerSeat(pub(crate) usize);
