use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct TestStep {
    #[arg]
    pub permanent: Entity,
    #[arg]
    pub controller: players::PlayerId,
}

impl Event for TestStep {
    const ID: rules::EventId = rules::EventId("test/step");
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

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
        assert_eq!(
            event.arg(TestStep::PERMANENT),
            Some(rules::ArgValue::Entity(permanent))
        );
        assert_eq!(
            event.arg(TestStep::CONTROLLER),
            Some(rules::ArgValue::Player(controller))
        );
        assert_eq!(TestStep::ID, rules::EventId("test/step"));
        assert_eq!(TestStep::restrictions(), Vec::new());
        assert_eq!(
            TestStep::args(),
            &[TestStep::PERMANENT, TestStep::CONTROLLER,]
        );
    }
}
