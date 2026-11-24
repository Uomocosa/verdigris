use crate::events;
use bevy::prelude::Entity;

use crate::players;

pub const fn from_controller(player: players::PlayerId, card: Entity) -> events::EnterBattlefield {
    events::EnterBattlefield {
        permanent: card,
        controller: player,
        summoning_sick: false,
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::testing::*;

    use super::from_controller;

    #[test]
    fn test_usage() {
        let (_game, [p1, ..]) = MockGame::new([P1, P2]);
        let event = from_controller(p1.id, Entity::PLACEHOLDER);
        assert_eq!(event.permanent, Entity::PLACEHOLDER);
        assert_eq!(event.controller, p1.id);
        assert!(!event.summoning_sick);
    }
}
