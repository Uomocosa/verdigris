use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::turns;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct AddPhase {
    #[arg]
    pub player: players::PlayerId,
    pub phase: turns::Phase,
}

impl Event for AddPhase {
    const ID: rules::EventId = rules::EventId("500.8/add-phase");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;
    use crate::turns;

    use super::AddPhase;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = AddPhase {
            player,
            phase: turns::Phase::Beginning,
        };
        assert_eq!(
            event.arg(AddPhase::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            event.arg(AddPhase::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(AddPhase::ID, rules::EventId("500.8/add-phase"));
        assert_eq!(AddPhase::restrictions(), Vec::new());
        assert_eq!(event.phase, turns::Phase::Beginning);
        assert_eq!(AddPhase::args(), &[AddPhase::PLAYER]);
    }
}
