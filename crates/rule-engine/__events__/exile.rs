use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Exile {
    #[arg]
    pub object: Entity,
}

impl Event for Exile {
    const ID: rules::EventId = rules::EventId("701.13a/exile");
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
        let event = Exile { object };
        assert_eq!(
            event.arg(Exile::OBJECT),
            Some(rules::ArgValue::Entity(object))
        );
        assert_eq!(Exile::ID, rules::EventId("701.13a/exile"));
        assert_eq!(Exile::restrictions(), Vec::new());
        assert_eq!(Exile::args(), &[Exile::OBJECT,]);
    }
}
