use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Destroy {
    #[arg]
    pub permanent: Entity,
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

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let event = Destroy { permanent };
        assert_eq!(
            event.arg(Destroy::PERMANENT),
            Some(rules::ArgValue::Entity(permanent))
        );
        assert_eq!(Destroy::ID, rules::EventId("701.8a/destroy"));
        assert_eq!(Destroy::restrictions(), Vec::new());
        assert_eq!(Destroy::args(), &[Destroy::PERMANENT,]);
    }
}
