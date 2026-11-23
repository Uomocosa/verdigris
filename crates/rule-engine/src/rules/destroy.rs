use bevy::prelude::Entity;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Destroy {
    pub permanent: Entity,
    pub controller: Entity,
}

#[rustfmt::skip]
impl rules::traits::context::Context for Destroy {
    fn subject(&self) -> Entity { self.permanent }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> Entity { self.controller }
}

impl Event for Destroy {
    const ID: rules::EventId = rules::EventId("701.8a");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::Destroy;

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let controller = Entity::from_raw_u32(1).unwrap();
        let event = Destroy {
            permanent,
            controller,
        };
        assert_eq!(event.subject(), permanent);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), controller);
        assert_eq!(Destroy::ID, rules::EventId("701.8a"));
        assert_eq!(Destroy::clauses(), Vec::new());
        assert_eq!(event.field(rules::Field::Amount), None);
    }
}
