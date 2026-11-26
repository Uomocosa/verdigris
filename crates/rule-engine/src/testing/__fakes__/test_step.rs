use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TestStep {
    pub permanent: Entity,
    pub controller: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for TestStep {
    fn subject(&self) -> Entity { self.permanent }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.controller }
}

impl Event for TestStep {
    const ID: rules::EventId = rules::EventId("test/step");
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::TestStep;
    use crate::players;

    #[test]
    fn test_usage() {
        use bevy::prelude::Entity;

        let permanent = Entity::PLACEHOLDER;
        let controller = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = TestStep {
            permanent,
            controller,
        };
        assert_eq!(event.subject(), permanent);
        assert_eq!(event.other(), None);
        assert_eq!(event.player(), controller);
        assert_eq!(TestStep::ID, rules::EventId("test/step"));
        assert_eq!(TestStep::restrictions(), Vec::new());
    }
}
