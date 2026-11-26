use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Discard {
    pub player: players::PlayerId,
    pub amount: u32,
}

#[rustfmt::skip]
impl rules::traits::context::Context for Discard {
    fn subject(&self) -> Entity { *self.player }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for Discard {
    const ID: rules::EventId = rules::EventId("701.9a/discard");

    fn field(&self, field: rules::Field) -> Option<u32> {
        match field {
            rules::Field::Amount => Some(self.amount),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::Discard;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = Discard { player, amount: 2 };
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(Discard::ID, rules::EventId("701.9a/discard"));
        assert_eq!(Discard::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), Some(2));
    }
}
