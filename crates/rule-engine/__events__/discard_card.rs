use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct DiscardCard {
    #[arg]
    pub card: Entity,
}

impl Event for DiscardCard {
    const ID: rules::EventId = rules::EventId("701.9a/discard-card");
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::DiscardCard;

    #[test]
    fn test_usage() {
        let card = Entity::PLACEHOLDER;
        let event = DiscardCard { card };
        assert_eq!(
            event.arg(DiscardCard::CARD),
            Some(rules::ArgValue::Entity(card))
        );
        assert_eq!(DiscardCard::ID, rules::EventId("701.9a/discard-card"));
        assert_eq!(DiscardCard::restrictions(), Vec::new());
        assert_eq!(DiscardCard::args(), &[DiscardCard::CARD,]);
    }
}
