use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::actions;
use crate::rules;
use crate::rules::traits::event::Event;

pub const DISCARD: actions::ChoiceId = actions::ChoiceId("701.9b/discard");

#[derive(Debug, Clone, PartialEq, Eq, Hash, EventArgs)]
pub struct Discard {
    #[arg]
    pub cards: Vec<Entity>,
    #[arg]
    pub cause: rules::EventId,
}

impl Event for Discard {
    const ID: rules::EventId = rules::EventId("701.9a/discard");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Discard;

    #[test]
    fn test_usage() {
        let card = Entity::PLACEHOLDER;
        let event = Discard {
            cards: vec![card],
            cause: rules::EventId("608.2/resolve"),
        };
        assert_eq!(
            event.arg(Discard::CARDS),
            Some(rules::ArgValue::Entities(vec![card]))
        );
        assert_eq!(
            event.arg(Discard::CAUSE),
            Some(rules::ArgValue::Event(rules::EventId("608.2/resolve")))
        );
        assert_eq!(Discard::ID, rules::EventId("701.9a/discard"));
        assert_eq!(Discard::restrictions(), Vec::new());
        assert_eq!(Discard::args(), &[Discard::CARDS, Discard::CAUSE,]);
    }
}
