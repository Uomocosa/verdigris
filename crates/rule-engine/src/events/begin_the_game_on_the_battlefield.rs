use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BeginTheGameOnTheBattlefield {
    pub card: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for BeginTheGameOnTheBattlefield {
    fn subject(&self) -> Entity { self.card }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for BeginTheGameOnTheBattlefield {
    const ID: rules::EventId = rules::EventId("103.6a/begin-the-game-on-the-battlefield");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::BeginTheGameOnTheBattlefield;

    #[test]
    fn test_usage() {
        let card = Entity::PLACEHOLDER;
        let player = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = BeginTheGameOnTheBattlefield { card, player };
        assert_eq!(event.subject(), card);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(
            BeginTheGameOnTheBattlefield::ID,
            rules::EventId("103.6a/begin-the-game-on-the-battlefield")
        );
        assert_eq!(BeginTheGameOnTheBattlefield::restrictions(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
    }
}
