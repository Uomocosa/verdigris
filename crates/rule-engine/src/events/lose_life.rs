use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoseLife {
    pub player: players::PlayerId,
    pub amount: u32,
}

#[rustfmt::skip]
impl rules::traits::context::Context for LoseLife {
    fn subject(&self) -> Entity { *self.player }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for LoseLife {
    const ID: rules::EventId = rules::EventId("119.3/lose");

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

    use super::LoseLife;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = LoseLife { player, amount: 2 };
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(LoseLife::ID, rules::EventId("119.3/lose"));
        assert_eq!(LoseLife::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), Some(2));
    }
}
