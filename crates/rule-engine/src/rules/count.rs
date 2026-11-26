use derive_more::Deref;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct Count(pub rules::ArgId);

#[rustfmt::skip]
impl Count {
    #[must_use]
    pub fn at_least(self, threshold: u32) -> rules::Condition {
        rules::Condition::Count(*self, rules::Comparison::AtLeast, threshold)
    }

    #[must_use]
    pub fn at_most(self, threshold: u32) -> rules::Condition {
        rules::Condition::Count(*self, rules::Comparison::AtMost, threshold)
    }

    #[must_use]
    pub fn equal_to(self, threshold: u32) -> rules::Condition {
        rules::Condition::Count(*self, rules::Comparison::EqualTo, threshold)
    }
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;

    use super::Count;

    #[test]
    fn test_usage() {
        let count = rules::count(events::DeclareAttackers::ATTACKERS);
        assert_eq!(*count, events::DeclareAttackers::ATTACKERS);
        assert_eq!(
            count.at_most(1),
            rules::Condition::Count(
                events::DeclareAttackers::ATTACKERS,
                rules::Comparison::AtMost,
                1
            )
        );
        assert_eq!(
            Count(events::DeclareAttackers::ATTACKERS).at_least(2),
            rules::Condition::Count(
                events::DeclareAttackers::ATTACKERS,
                rules::Comparison::AtLeast,
                2
            )
        );
        assert_eq!(
            Count(events::DeclareAttackers::ATTACKERS).equal_to(1),
            rules::Condition::Count(
                events::DeclareAttackers::ATTACKERS,
                rules::Comparison::EqualTo,
                1
            )
        );
    }
}
