use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiscardCard {
    pub card: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for DiscardCard {
    fn subject(&self) -> Entity { self.card }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for DiscardCard {
    const ID: rules::EventId = rules::EventId("701.9a");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::DiscardCard;

    #[test]
    fn test_usage() {
        let card = Entity::PLACEHOLDER;
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = DiscardCard { card, player };
        assert_eq!(event.subject(), card);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(DiscardCard::ID, rules::EventId("701.9a"));
        assert_eq!(DiscardCard::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
    }
}
