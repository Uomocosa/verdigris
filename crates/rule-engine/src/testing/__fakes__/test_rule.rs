use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::cards;
use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;
use crate::testing;

pub const UNTAPPED: rules::RestrictionId = rules::RestrictionId("test/untapped");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct TestRule {
    #[arg]
    pub attacker: Entity,
    #[arg]
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for TestRule {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

#[rustfmt::skip]
impl Event for TestRule {
    const ID: rules::EventId = rules::EventId("test/legality");
    fn restrictions() -> Vec<rules::Restriction> {
        vec![testing::UNTAPPED.restriction("must be untapped", !rules::has::<cards::Tapped>())]
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::TestRule;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(TestRule::ID, rules::EventId("test/legality"));
        let restrictions = TestRule::restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, UNTAPPED);
        assert_eq!(restrictions[0].condition, !rules::has::<cards::Tapped>());
        assert_eq!(TestRule::args(), &[TestRule::ATTACKER, TestRule::PLAYER,]);
    }
}
