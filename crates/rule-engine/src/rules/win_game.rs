use bevy::prelude::Entity;
use derive_more::Deref;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct WinGame(pub Entity);

#[rustfmt::skip]
impl Event for WinGame {
    type Legality = rules::WinsTheGame;
    const ID: rules::EventId = rules::EventId("104.2");
    fn context(self) -> rules::PlayerCtx { rules::PlayerCtx::from(*self) }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::WinGame;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let event = WinGame(player);
        assert_eq!(*event, player);
        assert_eq!(
            rules::traits::context::Context::player(&event.context()),
            player
        );
        assert_eq!(rules::WinsTheGame::ID, rules::RuleId("104.2"));
        assert_eq!(WinGame::ID, rules::EventId("104.2"));
    }
}
