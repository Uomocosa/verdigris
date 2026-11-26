use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct DrawFromEmptyLibrary {
    #[arg]
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for DrawFromEmptyLibrary {
    fn subject(&self) -> Entity { *self.player }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

#[rustfmt::skip]
impl Event for DrawFromEmptyLibrary {
    const ID: rules::EventId = rules::EventId("121.4/draw-from-empty-library");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::DrawFromEmptyLibrary;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = DrawFromEmptyLibrary { player };
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(
            DrawFromEmptyLibrary::ID,
            rules::EventId("121.4/draw-from-empty-library")
        );
        assert_eq!(DrawFromEmptyLibrary::restrictions(), Vec::new());
        assert_eq!(
            DrawFromEmptyLibrary::args(),
            &[DrawFromEmptyLibrary::PLAYER]
        );
    }
}
