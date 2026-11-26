use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

pub const ONLY_BLOCKERS: rules::RestrictionId = rules::RestrictionId("510.1c/only-blockers");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct AssignCombatDamage {
    #[arg]
    pub target: Entity,
    #[arg]
    pub source: Entity,
    #[arg]
    pub amount: u32,
}

impl Event for AssignCombatDamage {
    const ID: rules::EventId = rules::EventId("510.1c/assign-combat-damage");

    #[atomic_delegate(AssignCombatDamage, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::AssignCombatDamage;

    #[test]
    fn test_usage() {
        let target = Entity::from_raw_u32(1).unwrap();
        let source = Entity::PLACEHOLDER;
        let event = AssignCombatDamage {
            target,
            source,
            amount: 3,
        };
        assert_eq!(
            event.arg(AssignCombatDamage::TARGET),
            Some(rules::ArgValue::Entity(target))
        );
        assert_eq!(
            event.arg(AssignCombatDamage::SOURCE),
            Some(rules::ArgValue::Entity(source))
        );
        assert_eq!(
            AssignCombatDamage::ID,
            rules::EventId("510.1c/assign-combat-damage")
        );
        let restrictions = AssignCombatDamage::restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, super::ONLY_BLOCKERS);
        assert_eq!(
            restrictions[0].text,
            "A blocked creature assigns its combat damage to the creatures blocking it"
        );
        assert_eq!(
            restrictions[0].condition,
            rules::points_to::<cards::Blocking>(AssignCombatDamage::SOURCE)
                .of(AssignCombatDamage::TARGET)
        );
        assert_eq!(
            AssignCombatDamage::args(),
            &[
                AssignCombatDamage::TARGET,
                AssignCombatDamage::SOURCE,
                AssignCombatDamage::AMOUNT,
            ]
        );
    }

    #[test]
    fn restriction_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for restriction in AssignCombatDamage::restrictions() {
            assert!(
                RULES_TEXT.contains(restriction.text),
                "restriction {:?} text not found verbatim in Comprehensive Rules: {:?}",
                restriction.id,
                restriction.text
            );
        }
    }
}
