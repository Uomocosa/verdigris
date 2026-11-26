use atomic_delegate_macros::atomic_delegate;
use derive_more::Deref;

use crate::cards;
use crate::events;
use crate::rules;
use crate::rules::traits::hook_event::HookEvent;

pub const FROM_OPENING_HAND: rules::ClauseId<events::OpeningHandAction> =
    rules::ClauseId::new("103.6/from-opening-hand");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct OpeningHandAction(pub cards::CardId);

#[rustfmt::skip]
impl HookEvent for OpeningHandAction {
    const ID: rules::EventId = rules::EventId("103.6/opening-hand-action");

    fn card(&self) -> cards::CardId {
        **self
    }

    #[atomic_delegate(OpeningHandAction, HookEvent)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::hook_event::HookEvent;

    use super::FROM_OPENING_HAND;
    use super::OpeningHandAction;

    #[test]
    fn test_usage() {
        let card = cards::CardId(Entity::PLACEHOLDER);
        let event = OpeningHandAction(card);
        assert_eq!(*event, card);
        assert_eq!(event.card(), card);
        assert_eq!(
            OpeningHandAction::ID,
            rules::EventId("103.6/opening-hand-action")
        );
        let clauses = OpeningHandAction::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, FROM_OPENING_HAND.key);
        assert_eq!(
            clauses[0].text,
            "Some cards allow a player to take actions with them from their opening hand."
        );
        assert_eq!(clauses[0].role, rules::Role::Subject);
        assert_eq!(
            clauses[0].condition,
            rules::in_zone(HAND, rules::Role::Player)
        );
    }

    #[test]
    fn clause_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for clause in OpeningHandAction::clauses() {
            assert!(
                RULES_TEXT.contains(clause.text),
                "clause {:?} text not found verbatim in Comprehensive Rules: {:?}",
                clause.key,
                clause.text
            );
        }
    }
}
