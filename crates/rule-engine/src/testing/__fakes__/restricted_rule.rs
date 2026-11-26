use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RestrictedRule {
    pub attacker: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for RestrictedRule {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for RestrictedRule {
    const ID: rules::EventId = rules::EventId("test/restriction");

    #[atomic_delegate(RestrictedRule, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
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
        let restrictions = RestrictedRule::restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, testing::UNTAPPED);
    }
}
