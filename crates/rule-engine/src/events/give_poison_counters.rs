use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GivePoisonCounters {
    pub player: players::PlayerId,
    pub amount: u32,
}

#[rustfmt::skip]
impl rules::traits::context::Context for GivePoisonCounters {
    fn subject(&self) -> Entity { *self.player }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for GivePoisonCounters {
    const ID: rules::EventId = rules::EventId("122.1");

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

    use super::GivePoisonCounters;

    #[test]
    fn test_usage() {
        let player = players::PlayerId(Entity::PLACEHOLDER);
        let event = GivePoisonCounters { player, amount: 3 };
        assert_eq!(event.subject(), *player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(GivePoisonCounters::ID, rules::EventId("122.1"));
        assert_eq!(GivePoisonCounters::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), Some(3));
    }
}
