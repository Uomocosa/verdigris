use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Untap {
    #[arg]
    pub permanent: Entity,
    #[arg]
    pub cause: rules::EventId,
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

    #[test]
    fn test_usage() {
        let permanent = Entity::PLACEHOLDER;
        let event = Untap {
            permanent,
            cause: rules::EventId("502.3/untap-step"),
        };
        assert_eq!(
            event.arg(Untap::PERMANENT),
            Some(rules::ArgValue::Entity(permanent))
        );
        assert_eq!(
            event.arg(Untap::CAUSE),
            Some(rules::ArgValue::Event(rules::EventId("502.3/untap-step")))
        );
        assert_eq!(Untap::ID, rules::EventId("701.26b/untap"));
        assert_eq!(Untap::restrictions(), Vec::new());
        assert_eq!(Untap::args(), &[Untap::PERMANENT, Untap::CAUSE,]);
    }
}
