use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct LoseUnspentMana {
    #[arg]
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl Event for LoseUnspentMana {
    const ID: rules::EventId = rules::EventId("106.4/lose-unspent-mana");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::LoseUnspentMana;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = LoseUnspentMana { player };
        assert_eq!(
            event.arg(LoseUnspentMana::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            event.arg(LoseUnspentMana::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            LoseUnspentMana::ID,
            rules::EventId("106.4/lose-unspent-mana")
        );
        assert_eq!(LoseUnspentMana::restrictions(), Vec::new());
        assert_eq!(LoseUnspentMana::args(), &[LoseUnspentMana::PLAYER]);
    }
}
