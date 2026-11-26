use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;
use crate::testing;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct RestrictedRule {
    #[arg]
    pub attacker: Entity,
    #[arg]
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl Event for RestrictedRule {
    const ID: rules::EventId = rules::EventId("test/restriction");
    fn restrictions() -> Vec<rules::Restriction> {
        vec![testing::UNTAPPED.restriction(rules::Condition::True)]
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;
    use crate::testing;

    use super::RestrictedRule;

    #[test]
    fn test_usage() {
        assert_eq!(RestrictedRule::ID, rules::EventId("test/restriction"));
        let restrictions = RestrictedRule::restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, testing::UNTAPPED);
        assert_eq!(restrictions[0].condition, rules::Condition::True);
        assert_eq!(
            RestrictedRule::args(),
            &[RestrictedRule::ATTACKER, RestrictedRule::PLAYER,]
        );
    }
}
