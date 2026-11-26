use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Destroy {
    #[arg]
    pub permanent: Entity,
    #[arg]
    pub controller: players::PlayerId,
}

impl Event for Destroy {
    const ID: rules::EventId = rules::EventId("701.8a/destroy");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Destroy;
    use crate::players;

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let controller = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let event = Destroy {
            permanent,
            controller,
        };
        assert_eq!(
            event.arg(Destroy::PERMANENT),
            Some(rules::ArgValue::Entity(permanent))
        );
        assert_eq!(
            event.arg(Destroy::CONTROLLER),
            Some(rules::ArgValue::Player(controller))
        );
        assert_eq!(Destroy::ID, rules::EventId("701.8a/destroy"));
        assert_eq!(Destroy::restrictions(), Vec::new());
        assert_eq!(Destroy::args(), &[Destroy::PERMANENT, Destroy::CONTROLLER,]);
    }
}
