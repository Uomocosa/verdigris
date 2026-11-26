use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::turns;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AddPhase {
    pub player: players::PlayerId,
    pub phase: turns::Phase,
}

#[rustfmt::skip]
impl rules::traits::context::Context for AddPhase {
    fn subject(&self) -> Entity { *self.player }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for AddPhase {
    const ID: rules::EventId = rules::EventId("500.8/add-phase");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::turns;

    use super::AddPhase;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = AddPhase {
            player,
            phase: turns::Phase::Beginning,
        };
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(AddPhase::ID, rules::EventId("500.8/add-phase"));
        assert_eq!(AddPhase::restrictions(), Vec::new());
        assert_eq!(event.phase, turns::Phase::Beginning);
    }
}
