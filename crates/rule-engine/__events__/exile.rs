use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Exile {
    #[arg]
    pub object: Entity,
    #[arg]
    pub cause: rules::EventId,
}

impl Event for Exile {
    const ID: rules::EventId = rules::EventId("701.13a/exile");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::OBJECT);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Exile;

    #[test]
    fn test_usage() {
        let object = Entity::PLACEHOLDER;
        let event = Exile {
            object,
            cause: rules::EventId("608.2/resolve"),
        };
        assert_eq!(
            event.arg(Exile::OBJECT),
            Some(rules::ArgValue::Entity(object))
        );
        assert_eq!(
            event.arg(Exile::CAUSE),
            Some(rules::ArgValue::Event(rules::EventId("608.2/resolve")))
        );
        assert_eq!(Exile::ID, rules::EventId("701.13a/exile"));
        assert_eq!(Exile::HOLDER, rules::ArgRef::Arg(Exile::OBJECT));
        assert_eq!(Exile::restrictions(), Vec::new());
        assert_eq!(Exile::args(), &[Exile::OBJECT, Exile::CAUSE,]);
    }
}
