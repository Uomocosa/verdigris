use bevy::prelude::Entity;
use derive_more::Deref;

use crate::rules;
use crate::rules::traits::event::Event;

pub const DRAW_FROM_EMPTY_LIBRARY: rules::EventId = rules::EventId("121.4");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct DrawFromEmptyLibrary(pub Entity);

#[rustfmt::skip]
impl Event for DrawFromEmptyLibrary {
    type Legality = rules::DrawsFromEmptyLibrary;
    const ID: rules::EventId = DRAW_FROM_EMPTY_LIBRARY;
    fn context(self) -> rules::PlayerCtx { rules::PlayerCtx::from(*self) }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::DrawFromEmptyLibrary;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let event = DrawFromEmptyLibrary(player);
        assert_eq!(*event, player);
        assert_eq!(
            rules::traits::context::Context::player(&event.context()),
            player
        );
        assert_eq!(rules::DrawsFromEmptyLibrary::ID, rules::EventId("121.4"));
        assert_eq!(DrawFromEmptyLibrary::ID, rules::DRAW_FROM_EMPTY_LIBRARY);
    }
}
