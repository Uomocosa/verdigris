use bevy::prelude::Entity;
use derive_more::Deref;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct WinGame(pub Entity);

#[rustfmt::skip]
impl rules::traits::context::Context for WinGame {
    fn subject(&self) -> Entity { **self }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { **self }
}

#[rustfmt::skip]
impl Event for WinGame {
    const ID: rules::EventId = rules::EventId("104.2");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::WinGame;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let event = WinGame(player);
        assert_eq!(event.subject(), player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(WinGame::ID, rules::EventId("104.2"));
        assert_eq!(WinGame::clauses(), Vec::new());
    }
}
