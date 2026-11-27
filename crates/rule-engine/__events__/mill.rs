use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Mill {
    #[arg]
    pub player: players::PlayerId,
    #[arg]
    pub amount: u32,
    #[arg]
    pub cause: rules::EventId,
}

impl Event for Mill {
    const ID: rules::EventId = rules::EventId("701.17a/mill");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Mill;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = Mill {
            player,
            amount: 2,
            cause: rules::EventId("608.2/resolve"),
        };
        assert_eq!(
            event.arg(Mill::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            event.arg(Mill::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            event.arg(Mill::CAUSE),
            Some(rules::ArgValue::Event(rules::EventId("608.2/resolve")))
        );
        assert_eq!(Mill::ID, rules::EventId("701.17a/mill"));
        assert_eq!(Mill::restrictions(), Vec::new());
        assert_eq!(Mill::args(), &[Mill::PLAYER, Mill::AMOUNT, Mill::CAUSE,]);
    }
}
