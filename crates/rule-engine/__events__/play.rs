use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

pub const PRIORITY: rules::RestrictionId = rules::RestrictionId("305.1/priority");
pub const FROM_HAND: rules::RestrictionId = rules::RestrictionId("305.1/from-hand");
pub const LAND_CARD: rules::RestrictionId = rules::RestrictionId("305.1/land-card");
pub const MAIN_PHASE: rules::RestrictionId = rules::RestrictionId("305.1/main-phase");
pub const EMPTY_STACK: rules::RestrictionId = rules::RestrictionId("305.1/empty-stack");
pub const OWNS: rules::RestrictionId = rules::RestrictionId("305.1/owns");
pub const OWN_TURN: rules::RestrictionId = rules::RestrictionId("305.3/own-turn");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Play {
    #[arg]
    pub card: Entity,
}

impl Event for Play {
    const ID: rules::EventId = rules::EventId("305.1/play");

    #[atomic_delegate(Play, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::EMPTY_STACK;
    use super::FROM_HAND;
    use super::LAND_CARD;
    use super::MAIN_PHASE;
    use super::OWN_TURN;
    use super::OWNS;
    use super::PRIORITY;
    use super::Play;

    #[test]
    fn test_usage() {
        assert_eq!(Play::ID, rules::EventId("305.1/play"));
        let restrictions = Play::restrictions();
        assert_eq!(restrictions.len(), 7);
        assert_eq!(restrictions[0].id, PRIORITY);
        assert_eq!(
            restrictions[0].condition,
            rules::has_priority(Play::CARD.controller())
        );
        assert_eq!(restrictions[1].id, LAND_CARD);
        assert_eq!(
            restrictions[1].condition,
            rules::card_type(cards::CardType::Land).of(Play::CARD)
        );
        assert_eq!(restrictions[2].id, FROM_HAND);
        assert_eq!(
            restrictions[2].condition,
            rules::in_zone(HAND, Play::CARD.controller()).of(Play::CARD)
        );
        assert_eq!(restrictions[3].id, MAIN_PHASE);
        assert_eq!(restrictions[3].condition, rules::main_phase());
        assert_eq!(restrictions[4].id, EMPTY_STACK);
        assert_eq!(restrictions[4].condition, rules::stack_empty());
        assert_eq!(restrictions[5].id, OWNS);
        assert_eq!(
            restrictions[5].condition,
            rules::points_to::<cards::Owner>(Play::CARD.controller()).of(Play::CARD)
        );
        assert_eq!(restrictions[6].id, OWN_TURN);
        assert_eq!(
            restrictions[6].condition,
            rules::active_player(Play::CARD.controller())
        );
        assert!(restrictions[6].unwaivable);
        assert!(
            Play::restrictions()
                .iter()
                .filter(|restriction| restriction.id != OWN_TURN)
                .all(|restriction| !restriction.unwaivable)
        );
        assert_eq!(Play::args(), &[Play::CARD,]);
    }

    #[test]
    fn restriction_rule_numbers_occur_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../docs/MagicCompRules-20250919.txt");
        for restriction in Play::restrictions() {
            let id: &str = *restriction.id;
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
