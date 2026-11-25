use derive_more::Deref;

use crate::cards;
use crate::rules;
use crate::rules::traits::hook_event::HookEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct OpeningHandAction(pub cards::CardId);

#[rustfmt::skip]
impl HookEvent for OpeningHandAction {
    const ID: rules::EventId = rules::EventId("103.6");
    fn card(&self) -> cards::CardId { **self }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::rules;

    use super::HookEvent;
    use super::OpeningHandAction;

    #[test]
    fn test_usage() {
        let card = cards::CardId(Entity::PLACEHOLDER);
        let event = OpeningHandAction(card);
        assert_eq!(*event, card);
        assert_eq!(event.card(), card);
        assert_eq!(OpeningHandAction::ID, rules::EventId("103.6"));
        assert_eq!(OpeningHandAction::clauses(), Vec::new());
    }
}
