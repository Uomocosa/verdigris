use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AbsoluteRule {
    pub attacker: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for AbsoluteRule {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for AbsoluteRule {
    const ID: rules::EventId = rules::EventId("test/absolute");

    #[atomic_delegate(AbsoluteRule, Event)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::testing;

    use super::AbsoluteRule;

    #[test]
    fn test_usage() {
        assert_eq!(AbsoluteRule::ID, rules::EventId("test/absolute"));
        let clauses = AbsoluteRule::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, testing::UNTAPPED.key);
        assert!(clauses[0].absolute);
    }
}
