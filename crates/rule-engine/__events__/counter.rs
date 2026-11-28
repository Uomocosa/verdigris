use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Counter {
    #[arg]
    pub spell: Entity,
    #[arg]
    pub cause: rules::EventId,
}

impl Event for Counter {
    const ID: rules::EventId = rules::EventId("701.6a/counter");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::SPELL);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Counter;

    #[test]
    fn test_usage() {
        let spell = Entity::PLACEHOLDER;
        let event = Counter {
            spell,
            cause: rules::EventId("608.2/resolve"),
        };
        assert_eq!(
            event.arg(Counter::SPELL),
            Some(rules::ArgValue::Entity(spell))
        );
        assert_eq!(
            event.arg(Counter::CAUSE),
            Some(rules::ArgValue::Event(rules::EventId("608.2/resolve")))
        );
        assert_eq!(Counter::ID, rules::EventId("701.6a/counter"));
        assert_eq!(Counter::HOLDER, rules::ArgRef::Arg(Counter::SPELL));
        assert_eq!(Counter::restrictions(), Vec::new());
        assert_eq!(Counter::args(), &[Counter::SPELL, Counter::CAUSE,]);
    }
}
