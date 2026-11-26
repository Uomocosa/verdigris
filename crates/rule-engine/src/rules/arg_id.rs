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

    #[must_use]
    pub const fn at_least(self, threshold: u32) -> rules::Condition {
        rules::Condition::ArgAmount(self, rules::Comparison::AtLeast, threshold)
    }

    #[must_use]
    pub const fn at_most(self, threshold: u32) -> rules::Condition {
        rules::Condition::ArgAmount(self, rules::Comparison::AtMost, threshold)
    }

    #[must_use]
    pub const fn equal_to(self, threshold: u32) -> rules::Condition {
        rules::Condition::ArgAmount(self, rules::Comparison::EqualTo, threshold)
    }

    #[must_use]
    pub const fn greater_than(self, threshold: u32) -> rules::Condition {
        rules::Condition::ArgAmount(self, rules::Comparison::GreaterThan, threshold)
    }

    #[must_use]
    pub const fn less_than(self, threshold: u32) -> rules::Condition {
        rules::Condition::ArgAmount(self, rules::Comparison::LessThan, threshold)
    }
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
        assert_eq!(
            id.at_least(2),
            rules::Condition::ArgAmount(id, rules::Comparison::AtLeast, 2)
        );
        assert_eq!(
            id.greater_than(0),
            rules::Condition::ArgAmount(id, rules::Comparison::GreaterThan, 0)
        );
    }
}
