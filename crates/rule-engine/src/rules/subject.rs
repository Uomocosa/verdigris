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

#[rustfmt::skip]
impl From<rules::Each> for Subject {
    fn from(each: rules::Each) -> Self { Self::Each(*each) }
}

#[rustfmt::skip]
impl From<rules::An> for Subject {
    fn from(an: rules::An) -> Self { Self::An(*an) }
}
