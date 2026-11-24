use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DealDamage {
    pub source: Entity,
    pub recipient: Entity,
    pub amount: u32,
    pub controller: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for DealDamage {
    fn subject(&self) -> Entity { self.recipient }
    fn other(&self) -> Option<Entity> { Some(self.source) }
    fn player(&self) -> players::PlayerId { self.controller }
}

impl Event for DealDamage {
    const ID: rules::EventId = rules::EventId("120.3");

    fn field(&self, field: rules::Field) -> Option<u32> {
        match field {
            rules::Field::Amount => Some(self.amount),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::DealDamage;
    use crate::players;

    #[test]
    fn test_usage() {
        let source = Entity::PLACEHOLDER;
        let recipient = Entity::from_raw_u32(1).unwrap();
        let controller = players::PlayerId(Entity::from_raw_u32(2).unwrap());
        let event = DealDamage {
            source,
            recipient,
            amount: 3,
            controller,
        };
        assert_eq!(event.subject(), recipient);
        assert_eq!(event.other(), Some(source));
        assert_eq!(event.player(), controller);
        assert_eq!(DealDamage::ID, rules::EventId("120.3"));
        assert_eq!(DealDamage::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), Some(3));
    }
}
