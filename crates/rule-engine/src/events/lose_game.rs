use bevy::prelude::Entity;
use derive_more::Deref;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct LoseGame(pub players::PlayerId);

#[rustfmt::skip]
impl rules::traits::context::Context for LoseGame {
    fn subject(&self) -> Entity { ***self }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { **self }
}

#[rustfmt::skip]
impl Event for LoseGame {
    const ID: rules::EventId = rules::EventId("104.3/lose-game");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::LoseGame;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = LoseGame(player);
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(LoseGame::ID, rules::EventId("104.3/lose-game"));
        assert_eq!(LoseGame::clauses(), Vec::new());
    }
}
