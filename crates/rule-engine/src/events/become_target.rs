use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct BecomeTarget {
    #[arg]
    pub target: Entity,
    #[arg]
    pub spell: Entity,
    #[arg]
    pub player: players::PlayerId,
}

impl Event for BecomeTarget {
    const ID: rules::EventId = rules::EventId("601.2c/become-target");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::BecomeTarget;

    #[test]
    fn test_usage() {
        let target = Entity::PLACEHOLDER;
        let spell = Entity::from_raw_u32(1).unwrap();
        let player = players::PlayerId(Entity::from_raw_u32(2).unwrap());
        let event = BecomeTarget {
            target,
            spell,
            player,
        };
        assert_eq!(
            event.arg(BecomeTarget::TARGET),
            Some(rules::ArgValue::Entity(target))
        );
        assert_eq!(
            event.arg(BecomeTarget::SPELL),
            Some(rules::ArgValue::Entity(spell))
        );
        assert_eq!(
            event.arg(BecomeTarget::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(BecomeTarget::ID, rules::EventId("601.2c/become-target"));
        assert_eq!(BecomeTarget::restrictions(), Vec::new());
        assert_eq!(
            BecomeTarget::args(),
            &[
                BecomeTarget::TARGET,
                BecomeTarget::SPELL,
                BecomeTarget::PLAYER,
            ]
        );
    }
}
