use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Discard {
    #[arg]
    pub player: players::PlayerId,
    #[arg]
    pub amount: u32,
}

impl Event for Discard {
    const ID: rules::EventId = rules::EventId("701.9a/discard");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Discard;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = Discard { player, amount: 2 };
        assert_eq!(
            event.arg(Discard::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            event.arg(Discard::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(Discard::ID, rules::EventId("701.9a/discard"));
        assert_eq!(Discard::restrictions(), Vec::new());
        assert_eq!(Discard::args(), &[Discard::PLAYER, Discard::AMOUNT,]);
    }
}
