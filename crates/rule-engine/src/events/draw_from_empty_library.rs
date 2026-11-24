use bevy::prelude::Entity;
use derive_more::Deref;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct DrawFromEmptyLibrary(pub players::PlayerId);

#[rustfmt::skip]
impl rules::traits::context::Context for DrawFromEmptyLibrary {
    fn subject(&self) -> Entity { ***self }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { **self }
}

#[rustfmt::skip]
impl Event for DrawFromEmptyLibrary {
    const ID: rules::EventId = rules::EventId("121.4");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::DrawFromEmptyLibrary;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = DrawFromEmptyLibrary(player);
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(DrawFromEmptyLibrary::ID, rules::EventId("121.4"));
        assert_eq!(DrawFromEmptyLibrary::clauses(), Vec::new());
    }
}
