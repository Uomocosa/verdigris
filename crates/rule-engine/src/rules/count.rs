use derive_more::Deref;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct Count(pub rules::ArgId);

#[rustfmt::skip]
impl Count {
    #[must_use]
    pub fn is(self, comparison: rules::Comparison) -> rules::Condition {
        rules::Condition::Count(*self, comparison)
    }
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;

    use super::Count;

    #[test]
    fn test_usage() {
        let count = Count(events::DeclareAttackers::ATTACKERS);
        assert_eq!(*count, events::DeclareAttackers::ATTACKERS);
        assert_eq!(
            count.is(rules::Value(1).or_less()),
            rules::Condition::Count(
                events::DeclareAttackers::ATTACKERS,
                rules::Comparison::OrLess(1),
            ),
        );
    }
}
