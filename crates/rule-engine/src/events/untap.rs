use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Untap {
    #[arg]
    pub permanent: Entity,
    #[arg]
    pub controller: players::PlayerId,
}

impl Event for Untap {
    const ID: rules::EventId = rules::EventId("701.26b/untap");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

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
        assert_eq!(
            event.arg(Untap::PERMANENT),
            Some(rules::ArgValue::Entity(permanent))
        );
        assert_eq!(
            event.arg(Untap::CONTROLLER),
            Some(rules::ArgValue::Player(controller))
        );
        assert_eq!(Untap::ID, rules::EventId("701.26b/untap"));
        assert_eq!(Untap::restrictions(), Vec::new());
        assert_eq!(Untap::args(), &[Untap::PERMANENT, Untap::CONTROLLER,]);
    }
}
