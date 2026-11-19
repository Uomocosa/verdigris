use derive_more::{Deref, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deref, Display)]
pub struct TestCardName(pub(crate) &'static str);
