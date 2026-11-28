use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct LoseLife {
    #[arg]
    pub player: players::PlayerId,
    #[arg]
    pub amount: u32,
}

impl Event for LoseLife {
    const ID: rules::EventId = rules::EventId("119.3/lose-life");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::PLAYER);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::LoseLife;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = LoseLife { player, amount: 2 };
        assert_eq!(
            event.arg(LoseLife::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            event.arg(LoseLife::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(LoseLife::ID, rules::EventId("119.3/lose-life"));
        assert_eq!(LoseLife::HOLDER, rules::ArgRef::Arg(LoseLife::PLAYER));
        assert_eq!(LoseLife::restrictions(), Vec::new());
        assert_eq!(LoseLife::args(), &[LoseLife::PLAYER, LoseLife::AMOUNT,]);
    }
}
