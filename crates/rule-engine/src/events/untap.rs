use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Untap {
    pub permanent: Entity,
    pub controller: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for Untap {
    fn subject(&self) -> Entity { self.permanent }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.controller }
}

impl Event for Untap {
    const ID: rules::EventId = rules::EventId("701.26b");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::Untap;
    use crate::players;

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let controller = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = Untap {
            permanent,
            controller,
        };
        assert_eq!(event.subject(), permanent);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), controller);
        assert_eq!(Untap::ID, rules::EventId("701.26b"));
        assert_eq!(Untap::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
    }
}
