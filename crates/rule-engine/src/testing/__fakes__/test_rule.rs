use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const UNTAPPED: rules::RestrictionId = rules::RestrictionId("test/untapped");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestRule {
    pub attacker: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for TestRule {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for TestRule {
    const ID: rules::EventId = rules::EventId("test/legality");

    #[atomic_delegate(TestRule, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::TestRule;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(TestRule::ID, rules::EventId("test/legality"));
        let restrictions = TestRule::restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, UNTAPPED);
    }
}
