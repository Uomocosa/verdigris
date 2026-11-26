use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const PRIORITY: rules::RestrictionId = rules::RestrictionId("601.2a/priority");
pub const FROM_HAND: rules::RestrictionId = rules::RestrictionId("601.2a/from-hand");
pub const OWNS: rules::RestrictionId = rules::RestrictionId("601.2a/owns");
pub const NOT_A_LAND: rules::RestrictionId = rules::RestrictionId("601.2a/not-a-land");
pub const TIMING: rules::RestrictionId = rules::RestrictionId("601.2a/timing");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct CastSpell {
    #[arg]
    pub spell: Entity,
    #[arg]
    pub player: players::PlayerId,
}

impl Event for CastSpell {
    const ID: rules::EventId = rules::EventId("601.2a/cast-spell");

    #[atomic_delegate(CastSpell, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::CastSpell;
    use super::FROM_HAND;
    use super::NOT_A_LAND;
    use super::OWNS;
    use super::PRIORITY;
    use super::TIMING;

    #[test]
    fn test_usage() {
        assert_eq!(CastSpell::ID, rules::EventId("601.2a/cast-spell"));
        let restrictions = CastSpell::restrictions();
        assert_eq!(restrictions.len(), 5);
        assert_eq!(restrictions[0].id, PRIORITY);
        assert_eq!(
            restrictions[0].condition,
            rules::has_priority(CastSpell::PLAYER)
        );
        assert_eq!(restrictions[1].id, FROM_HAND);
        assert_eq!(
            restrictions[1].condition,
            rules::in_zone(HAND, CastSpell::PLAYER).of(CastSpell::SPELL)
        );
        assert_eq!(restrictions[2].id, OWNS);
        assert_eq!(
            restrictions[2].condition,
            rules::points_to::<cards::Owner>(CastSpell::PLAYER).of(CastSpell::SPELL)
        );
        assert_eq!(restrictions[3].id, NOT_A_LAND);
        assert_eq!(
            restrictions[3].condition,
            (!rules::card_type(cards::CardType::Land)).of(CastSpell::SPELL)
        );
        assert!(restrictions[3].unwaivable);
        assert_eq!(restrictions[4].id, TIMING);
        assert_eq!(
            restrictions[4].condition,
            rules::card_type(cards::CardType::Instant).of(CastSpell::SPELL)
                | (rules::main_phase()
                    & rules::stack_empty()
                    & rules::active_player(CastSpell::PLAYER))
        );
        assert!(
            CastSpell::restrictions()
                .iter()
                .filter(|restriction| restriction.id != NOT_A_LAND)
                .all(|restriction| !restriction.unwaivable)
        );
        assert_eq!(CastSpell::args(), &[CastSpell::SPELL, CastSpell::PLAYER,]);
    }

    #[test]
    fn restriction_rule_numbers_occur_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for restriction in CastSpell::restrictions() {
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
