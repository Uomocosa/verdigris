use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::counters;
use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct PutCounters {
    #[arg]
    pub recipient: Entity,
    pub kind: counters::CounterId,
    #[arg]
    pub amount: u32,
}

impl Event for PutCounters {
    const ID: rules::EventId = rules::EventId("122.6/put-counters");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::RECIPIENT);
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::counters;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::PutCounters;

    #[test]
    fn test_usage() {
        let recipient = Entity::PLACEHOLDER;
        let event = PutCounters {
            recipient,
            kind: counters::POISON,
            amount: 3,
        };
        assert_eq!(
            event.arg(PutCounters::RECIPIENT),
            Some(rules::ArgValue::Entity(recipient))
        );
        assert_eq!(
            event.arg(PutCounters::AMOUNT),
            Some(rules::ArgValue::Amount(3))
        );
        assert_eq!(PutCounters::ID, rules::EventId("122.6/put-counters"));
        assert_eq!(
            PutCounters::HOLDER,
            rules::ArgRef::Arg(PutCounters::RECIPIENT)
        );
        assert_eq!(PutCounters::restrictions(), Vec::new());
        assert_eq!(
            PutCounters::args(),
            &[PutCounters::RECIPIENT, PutCounters::AMOUNT,]
        );
    }
}
