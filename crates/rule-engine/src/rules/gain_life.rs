use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GainLife {
    pub player: Entity,
    pub amount: u32,
}

#[rustfmt::skip]
impl rules::traits::context::Context for GainLife {
    fn subject(&self) -> Entity { self.player }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { self.player }
}

impl Event for GainLife {
    const ID: rules::EventId = rules::EventId("119.3");

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

    use super::GainLife;

    #[test]
    fn test_usage() {
        let player = Entity::PLACEHOLDER;
        let event = GainLife { player, amount: 2 };
        assert_eq!(event.subject(), player);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), player);
        assert_eq!(GainLife::ID, rules::EventId("119.3"));
        assert_eq!(GainLife::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), Some(2));
    }
}
