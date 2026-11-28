use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::counters;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct RemoveCounters {
    #[arg]
    pub recipient: Entity,
    pub kind: counters::CounterId,
    #[arg]
    pub amount: u32,
    #[arg]
    pub cause: rules::EventId,
}

impl Event for RemoveCounters {
    const ID: rules::EventId = rules::EventId("122.3/remove-counters");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::RECIPIENT);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::PLUS_ONE_PLUS_ONE;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::RemoveCounters;

    #[test]
    fn test_usage() {
        let recipient = Entity::PLACEHOLDER;
        let cause = rules::EventId("704.5q/counter-cancellation");
        let event = RemoveCounters {
            recipient,
            kind: PLUS_ONE_PLUS_ONE,
            amount: 1,
            cause,
        };
        assert_eq!(
            event.arg(RemoveCounters::RECIPIENT),
            Some(rules::ArgValue::Entity(recipient))
        );
        assert_eq!(
            event.arg(RemoveCounters::AMOUNT),
            Some(rules::ArgValue::Amount(1))
        );
        assert_eq!(
            event.arg(RemoveCounters::CAUSE),
            Some(rules::ArgValue::Event(cause))
        );
        assert_eq!(RemoveCounters::ID, rules::EventId("122.3/remove-counters"));
        assert_eq!(
            RemoveCounters::HOLDER,
            rules::ArgRef::Arg(RemoveCounters::RECIPIENT)
        );
        assert_eq!(RemoveCounters::restrictions(), Vec::new());
        assert_eq!(
            RemoveCounters::args(),
            &[
                RemoveCounters::RECIPIENT,
                RemoveCounters::AMOUNT,
                RemoveCounters::CAUSE,
            ]
        );
    }
}
