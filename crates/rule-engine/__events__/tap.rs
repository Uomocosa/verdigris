use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Tap {
    #[arg]
    pub permanent: Entity,
}

impl Event for Tap {
    const ID: rules::EventId = rules::EventId("701.26a/tap");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Tap;

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let event = Tap { permanent };
        assert_eq!(
            event.arg(Tap::PERMANENT),
            Some(rules::ArgValue::Entity(permanent))
        );
        assert_eq!(Tap::ID, rules::EventId("701.26a/tap"));
        assert_eq!(Tap::restrictions(), Vec::new());
        assert_eq!(Tap::args(), &[Tap::PERMANENT,]);
    }
}
