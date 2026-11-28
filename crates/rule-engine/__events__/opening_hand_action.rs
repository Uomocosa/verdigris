use atomic_delegate_macros::atomic_delegate;
use verdigris_event_derive::EventArgs;

use crate::cards;
use crate::rules;
use crate::rules::traits::hook_event::HookEvent;

pub const FROM_OPENING_HAND: rules::RestrictionId =
    rules::RestrictionId::rule("103.6/from-opening-hand");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct OpeningHandAction {
    #[arg]
    pub card: cards::CardId,
}

#[rustfmt::skip]
impl HookEvent for OpeningHandAction {
    const ID: rules::EventId = rules::EventId("103.6/opening-hand-action");

    fn card(&self) -> cards::CardId {
        self.card
    }

    #[atomic_delegate(OpeningHandAction, HookEvent)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::event_args::EventArgs;
    use crate::rules::traits::hook_event::HookEvent;

    use super::FROM_OPENING_HAND;
    use super::OpeningHandAction;

    #[test]
    fn test_usage() {
        let card = cards::CardId(Entity::PLACEHOLDER);
        let event = OpeningHandAction { card };
        assert_eq!(event.card(), card);
        assert_eq!(
            OpeningHandAction::ID,
            rules::EventId("103.6/opening-hand-action")
        );
        let restrictions = OpeningHandAction::restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, FROM_OPENING_HAND);
        assert_eq!(
            restrictions[0].condition,
            rules::in_zone(HAND, rules::Hooked::<super::OpeningHandAction>::PLAYER,)
                .of(rules::Hooked::<super::OpeningHandAction>::CARD)
        );
        assert_eq!(OpeningHandAction::args(), &[OpeningHandAction::CARD]);
    }

    #[test]
    fn restriction_rule_numbers_occur_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../docs/MagicCompRules-20250919.txt");
        for restriction in OpeningHandAction::restrictions() {
            let id = restriction.id.to_string();
            if id.starts_with("test/") || id.starts_with("statics/") || id.starts_with("glossary/")
            {
                continue;
            }
            let (rule_number, _) = id.split_once('/').expect("restriction id cites its rule");
            assert!(
                RULES_TEXT.lines().any(|line| line.starts_with(&format!("{rule_number} "))
                    || line.starts_with(&format!("{rule_number}."))),
                "restriction {:?} rule number not found in Comprehensive Rules",
                restriction.id,
            );
        }
    }
}
