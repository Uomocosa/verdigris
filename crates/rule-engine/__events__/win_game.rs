use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct WinGame {
    #[arg]
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl Event for WinGame {
    const ID: rules::EventId = rules::EventId("104.2/win-game");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::PLAYER);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::WinGame;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = WinGame { player };
        assert_eq!(
            event.arg(WinGame::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            event.arg(WinGame::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(WinGame::ID, rules::EventId("104.2/win-game"));
        assert_eq!(WinGame::HOLDER, rules::ArgRef::Arg(WinGame::PLAYER));
        assert_eq!(WinGame::restrictions(), Vec::new());
        assert_eq!(WinGame::args(), &[WinGame::PLAYER]);
    }
}
