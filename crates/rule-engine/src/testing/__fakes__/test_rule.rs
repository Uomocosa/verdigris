use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event::Event;

pub const UNTAPPED: rules::ClauseId<TestRule> = rules::ClauseId::new("test/untapped");
pub const TEST_STEP: rules::ActionId = rules::ActionId("test/step");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestRule {
    pub attacker: Entity,
    pub player: Entity,
}

#[rustfmt::skip]
impl rules::traits::context::Context for TestRule {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { self.player }
}

impl Event for TestRule {
    const ID: rules::EventId = rules::EventId("test/legality");

    #[atomic_delegate(TestRule, Event)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::TEST_STEP;
    use super::TestRule;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(TestRule::ID, rules::EventId("test/legality"));
        assert_eq!(*TEST_STEP, "test/step");
        let clauses = TestRule::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, UNTAPPED.key);
    }
}
