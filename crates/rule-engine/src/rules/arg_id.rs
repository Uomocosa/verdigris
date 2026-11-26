use derive_more::Deref;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct ArgId(pub &'static str);

#[rustfmt::skip]
impl ArgId {
    #[must_use]
    pub const fn controller(self) -> rules::ArgRef { rules::ArgRef::ControllerOf(self) }

    #[must_use]
    pub const fn owner(self) -> rules::ArgRef { rules::ArgRef::OwnerOf(self) }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::ArgId;

    #[test]
    fn test_usage() {
        let id = ArgId("source");
        assert_eq!(id.controller(), rules::ArgRef::ControllerOf(id));
        assert_eq!(id.owner(), rules::ArgRef::OwnerOf(id));
    }
}
