use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Destroy {
    #[arg]
    pub permanent: Entity,
    #[arg]
    pub cause: rules::EventId,
}

impl Event for Destroy {
    const ID: rules::EventId = rules::EventId("701.8a/destroy");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::PERMANENT);
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
        let event = Destroy {
            permanent,
            cause: rules::EventId("704.5g/lethal-damage"),
        };
        assert_eq!(
            event.arg(Destroy::PERMANENT),
            Some(rules::ArgValue::Entity(permanent))
        );
        assert_eq!(
            event.arg(Destroy::CAUSE),
            Some(rules::ArgValue::Event(rules::EventId(
                "704.5g/lethal-damage"
            )))
        );
        assert_eq!(Destroy::ID, rules::EventId("701.8a/destroy"));
        assert_eq!(Destroy::HOLDER, rules::ArgRef::Arg(Destroy::PERMANENT));
        assert_eq!(Destroy::restrictions(), Vec::new());
        assert_eq!(Destroy::args(), &[Destroy::PERMANENT, Destroy::CAUSE,]);
    }
}
