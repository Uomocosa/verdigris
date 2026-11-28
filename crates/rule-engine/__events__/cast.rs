use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

pub const PRIORITY: rules::RestrictionId = rules::RestrictionId::rule("601.2a/priority");
pub const FROM_HAND: rules::RestrictionId = rules::RestrictionId::rule("601.2a/from-hand");
pub const OWNS: rules::RestrictionId = rules::RestrictionId::rule("601.2a/owns");
pub const NOT_A_LAND: rules::RestrictionId = rules::RestrictionId::rule("601.2a/not-a-land");
pub const TIMING: rules::RestrictionId = rules::RestrictionId::rule("601.2a/timing");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Cast {
    #[arg]
    pub spell: Entity,
    #[arg]
    pub caster: players::PlayerId,
}

impl Event for Cast {
    const ID: rules::EventId = rules::EventId("601.2a/cast");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::CASTER);

    #[atomic_delegate(Cast, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Cast;
    use super::FROM_HAND;
    use super::NOT_A_LAND;
    use super::OWNS;
    use super::PRIORITY;
    use super::TIMING;

    #[test]
    fn test_usage() {
        assert_eq!(Cast::ID, rules::EventId("601.2a/cast"));
        assert_eq!(Cast::HOLDER, rules::ArgRef::Arg(Cast::CASTER));
        let restrictions = Cast::restrictions();
        assert_eq!(restrictions.len(), 5);
        assert_eq!(restrictions[0].id, PRIORITY);
        assert_eq!(restrictions[0].condition, rules::has_priority(Cast::CASTER));
        assert_eq!(restrictions[1].id, FROM_HAND);
        assert_eq!(
            restrictions[1].condition,
            rules::in_zone(HAND, Cast::CASTER).of(Cast::SPELL)
        );
        assert_eq!(restrictions[2].id, OWNS);
        assert_eq!(
            restrictions[2].condition,
            rules::points_to::<cards::Owner>(Cast::CASTER).of(Cast::SPELL)
        );
        assert_eq!(restrictions[3].id, NOT_A_LAND);
        assert_eq!(
            restrictions[3].condition,
            (!rules::card_type(cards::CardType::Land)).of(Cast::SPELL)
        );
        assert!(restrictions[3].unwaivable);
        assert_eq!(restrictions[4].id, TIMING);
        assert_eq!(
            restrictions[4].condition,
            rules::card_type(cards::CardType::Instant).of(Cast::SPELL)
                | (rules::main_phase() & rules::stack_empty() & rules::active_player(Cast::CASTER))
        );
        assert!(
            Cast::restrictions()
                .iter()
                .filter(|restriction| restriction.id != NOT_A_LAND)
                .all(|restriction| !restriction.unwaivable)
        );
        assert_eq!(Cast::args(), &[Cast::SPELL, Cast::CASTER,]);
    }

    #[test]
    fn restriction_rule_numbers_occur_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../docs/MagicCompRules-20250919.txt");
        for restriction in Cast::restrictions() {
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
