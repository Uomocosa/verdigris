use bevy::prelude::Entity;
use derive_more::Deref;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct LoseGame(pub Entity);

#[rustfmt::skip]
impl Event for LoseGame {
    type Legality = rules::LosesTheGame;
    const ID: rules::EventId = rules::EventId("104.3");
    fn context(self) -> rules::PlayerCtx { rules::PlayerCtx::from(*self) }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::LoseGame;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let event = LoseGame(player);
        assert_eq!(*event, player);
        assert_eq!(
            rules::traits::context::Context::player(&event.context()),
            player
        );
        assert_eq!(rules::LosesTheGame::ID, rules::RuleId("104.3"));
        assert_eq!(LoseGame::ID, rules::EventId("104.3"));
    }
}
