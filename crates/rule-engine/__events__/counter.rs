use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Counter {
    #[arg]
    pub spell: Entity,
}

impl Event for Counter {
    const ID: rules::EventId = rules::EventId("701.6a/counter");
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
        let event = Counter { spell };
        assert_eq!(
            event.arg(Counter::SPELL),
            Some(rules::ArgValue::Entity(spell))
        );
        assert_eq!(Counter::ID, rules::EventId("701.6a/counter"));
        assert_eq!(Counter::restrictions(), Vec::new());
        assert_eq!(Counter::args(), &[Counter::SPELL,]);
    }
}
