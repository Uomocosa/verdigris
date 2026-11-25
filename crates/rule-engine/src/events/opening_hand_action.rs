use bevy::prelude::Entity;

use crate::cards;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OpeningHandAction {
    pub card: cards::CardId,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for OpeningHandAction {
    fn subject(&self) -> Entity { *self.card }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for OpeningHandAction {
    const ID: rules::EventId = rules::EventId("103.6");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::OpeningHandAction;

    #[test]
    fn test_usage() {
        let card = cards::CardId(Entity::PLACEHOLDER);
        let player = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = OpeningHandAction { card, player };
        assert_eq!(event.subject(), *card);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(OpeningHandAction::ID, rules::EventId("103.6"));
        assert_eq!(OpeningHandAction::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
    }
}
