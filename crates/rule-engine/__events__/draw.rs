use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Draw {
    #[arg]
    pub player: players::PlayerId,
    #[arg]
    pub amount: u32,
}

impl Event for Draw {
    const ID: rules::EventId = rules::EventId("121.1/draw");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Draw;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = Draw { player, amount: 2 };
        assert_eq!(
            event.arg(Draw::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            event.arg(Draw::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(Draw::ID, rules::EventId("121.1/draw"));
        assert_eq!(Draw::restrictions(), Vec::new());
        assert_eq!(Draw::args(), &[Draw::PLAYER, Draw::AMOUNT,]);
    }
}
