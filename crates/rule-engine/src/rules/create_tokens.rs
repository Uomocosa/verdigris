use bevy::prelude::Entity;

use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CreateTokens {
    pub controller: Entity,
    pub amount: u32,
    pub token: effects::TokenDefinition,
}

#[rustfmt::skip]
impl rules::traits::context::Context for CreateTokens {
    fn subject(&self) -> Entity { self.controller }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { self.controller }
}

impl Event for CreateTokens {
    const ID: rules::EventId = rules::EventId("701.7a");

    fn field(&self, field: rules::Field) -> Option<u32> {
        match field {
            rules::Field::Amount => Some(self.amount),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::effects;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::testing;

    use super::CreateTokens;

    #[test]
    fn test_usage() {
        let controller = Entity::PLACEHOLDER;
        let event = CreateTokens {
            controller,
            amount: 2,
            token: effects::TokenDefinition::from(testing::SOLDIER_TOKEN.definition),
        };
        assert_eq!(event.subject(), controller);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), controller);
        assert_eq!(CreateTokens::ID, rules::EventId("701.7a"));
        assert_eq!(CreateTokens::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), Some(2));
    }
}
