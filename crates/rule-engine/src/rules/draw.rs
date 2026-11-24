use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Draw {
    pub player: players::PlayerId,
    pub amount: u32,
}

#[rustfmt::skip]
impl rules::traits::context::Context for Draw {
    fn subject(&self) -> Entity { *self.player }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { *self.player }
}

impl Event for Draw {
    const ID: rules::EventId = rules::EventId("121.1");

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

    use super::Draw;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = Draw { player, amount: 2 };
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), *player);
        assert_eq!(Draw::ID, rules::EventId("121.1"));
        assert_eq!(Draw::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), Some(2));
    }
}
