use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Counter {
    pub spell: Entity,
    pub controller: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for Counter {
    fn subject(&self) -> Entity { self.spell }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.controller }
}

impl Event for Counter {
    const ID: rules::EventId = rules::EventId("701.6a");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::Counter;
    use crate::players;

    #[test]
    fn test_usage() {
        let spell = Entity::PLACEHOLDER;
        let controller = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = Counter { spell, controller };
        assert_eq!(event.subject(), spell);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), controller);
        assert_eq!(Counter::ID, rules::EventId("701.6a"));
        assert_eq!(Counter::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
    }
}
