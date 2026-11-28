use derive_more::Deref;

use crate::keywords;
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
    pub fn has(self, keyword: &keywords::Keyword) -> rules::Condition {
        rules::has_keyword(keyword.id).of(self)
    }

    #[must_use]
    pub const fn is(self, comparison: rules::Comparison) -> rules::Condition {
        rules::Condition::ArgAmount(self, comparison)
    }
}

#[cfg(test)]
mod tests {
    use crate::keywords;
    use crate::rules;

    use super::ArgId;

    #[test]
    fn test_usage() {
        let id = ArgId("spell_or_ability");
        assert_eq!(id.controller(), rules::ArgRef::ControllerOf(id));
        assert_eq!(id.owner(), rules::ArgRef::OwnerOf(id));
        assert_eq!(
            id.has(&keywords::flying()),
            rules::has_keyword(keywords::flying::ID).of(id)
        );
        assert_eq!(
            id.is(rules::Value(2).or_more()),
            rules::Condition::ArgAmount(id, rules::Comparison::OrMore(2)),
        );
    }
}
