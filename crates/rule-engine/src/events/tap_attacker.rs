use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct TapAttacker {
    #[arg]
    pub attacker: Entity,
    #[arg]
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for TapAttacker {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for TapAttacker {
    const ID: rules::EventId = rules::EventId("508.1f/tap-attacker");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::TapAttacker;
    use crate::players;

    #[test]
    fn test_usage() {
        let attacker = Entity::PLACEHOLDER;
        let player = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = TapAttacker { attacker, player };
        assert_eq!(event.subject(), attacker);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(TapAttacker::ID, rules::EventId("508.1f/tap-attacker"));
        assert_eq!(TapAttacker::restrictions(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
        assert_eq!(
            TapAttacker::args(),
            &[TapAttacker::ATTACKER, TapAttacker::PLAYER,]
        );
    }
}
