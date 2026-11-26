use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct BeginTheGameOnTheBattlefield {
    #[arg]
    pub card: Entity,
    #[arg]
    pub player: players::PlayerId,
}

impl Event for BeginTheGameOnTheBattlefield {
    const ID: rules::EventId = rules::EventId("103.6a/begin-the-game-on-the-battlefield");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::BeginTheGameOnTheBattlefield;

    #[test]
    fn test_usage() {
        let card = Entity::PLACEHOLDER;
        let player = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = BeginTheGameOnTheBattlefield { card, player };
        assert_eq!(
            event.arg(BeginTheGameOnTheBattlefield::CARD),
            Some(rules::ArgValue::Entity(card))
        );
        assert_eq!(
            event.arg(BeginTheGameOnTheBattlefield::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            BeginTheGameOnTheBattlefield::ID,
            rules::EventId("103.6a/begin-the-game-on-the-battlefield")
        );
        assert_eq!(BeginTheGameOnTheBattlefield::restrictions(), Vec::new());
        assert_eq!(
            BeginTheGameOnTheBattlefield::args(),
            &[
                BeginTheGameOnTheBattlefield::CARD,
                BeginTheGameOnTheBattlefield::PLAYER,
            ]
        );
    }
}
