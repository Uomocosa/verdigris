use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TapAttacker {
    pub attacker: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for TapAttacker {
    fn subject(&self) -> Entity { self.attacker }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for TapAttacker {
    const ID: rules::EventId = rules::EventId("508.1f");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

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
        assert_eq!(TapAttacker::ID, rules::EventId("508.1f"));
        assert_eq!(TapAttacker::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
    }
}
