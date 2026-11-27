use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct Count(pub rules::ArgId);

#[atomic_delegates]
impl Count {
    #[must_use]
    pub fn at_least(self, threshold: u32) -> rules::Condition {}

    #[must_use]
    pub fn at_most(self, threshold: u32) -> rules::Condition {}

    #[must_use]
    pub fn equal_to(self, threshold: u32) -> rules::Condition {}
}

#[cfg(test)]
mod tests {
    use crate::events;

    use super::Count;

    #[test]
    fn test_usage() {
        let count = Count(events::DeclareAttackers::ATTACKERS);
        assert_eq!(*count, events::DeclareAttackers::ATTACKERS);
    }
}
