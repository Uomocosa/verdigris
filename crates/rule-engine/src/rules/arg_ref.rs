use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArgRef {
    Arg(rules::ArgId),
    ControllerOf(rules::ArgId),
    OwnerOf(rules::ArgId),
}

#[rustfmt::skip]
impl From<rules::ArgId> for ArgRef {
    fn from(id: rules::ArgId) -> Self { Self::Arg(id) }
}
