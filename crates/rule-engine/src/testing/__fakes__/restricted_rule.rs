use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RestrictedRule {
    pub attacker: Entity,
    pub player: Entity,
}

#[rustfmt::skip]
impl rules::traits::context::Context for RestrictedRule {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { self.player }
}

impl Event for RestrictedRule {
    const ID: rules::EventId = rules::EventId("test/restriction");

    #[atomic_delegate(RestrictedRule, Event)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::testing;

    use super::RestrictedRule;

    #[test]
    fn test_usage() {
        assert_eq!(RestrictedRule::ID, rules::EventId("test/restriction"));
        let clauses = RestrictedRule::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, testing::UNTAPPED.key);
    }
}
