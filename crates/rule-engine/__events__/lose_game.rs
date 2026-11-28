use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct LoseGame {
    #[arg]
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl Event for LoseGame {
    const ID: rules::EventId = rules::EventId("104.3/lose-game");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::PLAYER);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::LoseGame;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = LoseGame { player };
        assert_eq!(
            event.arg(LoseGame::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            event.arg(LoseGame::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(LoseGame::ID, rules::EventId("104.3/lose-game"));
        assert_eq!(LoseGame::HOLDER, rules::ArgRef::Arg(LoseGame::PLAYER));
        assert_eq!(LoseGame::restrictions(), Vec::new());
        assert_eq!(LoseGame::args(), &[LoseGame::PLAYER]);
    }
}
