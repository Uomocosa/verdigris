use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subject {
    One(rules::ArgRef),
    Each(rules::Players),
    An(rules::Players),
}

#[rustfmt::skip]
impl From<rules::ArgRef> for Subject {
    fn from(reference: rules::ArgRef) -> Self { Self::One(reference) }
}

#[rustfmt::skip]
impl From<rules::ArgId> for Subject {
    fn from(id: rules::ArgId) -> Self { Self::One(rules::ArgRef::Arg(id)) }
}
