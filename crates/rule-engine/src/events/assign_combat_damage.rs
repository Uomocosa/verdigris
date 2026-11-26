use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const ONLY_BLOCKERS: rules::RestrictionId = rules::RestrictionId("510.1c/only-blockers");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssignCombatDamage {
    pub target: Entity,
    pub source: Entity,
    pub amount: u32,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for AssignCombatDamage {
    fn subject(&self) -> Entity { self.target }
    fn other(&self) -> Option<Entity> { Some(self.source) }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for AssignCombatDamage {
    const ID: rules::EventId = rules::EventId("510.1c/assign-combat-damage");

    fn field(&self, field: rules::Field) -> Option<u32> {
        match field {
            rules::Field::Amount => Some(self.amount),
        }
    }

    #[atomic_delegate(AssignCombatDamage, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::AssignCombatDamage;

    #[test]
    fn test_usage() {
        let target = Entity::from_raw_u32(1).unwrap();
        let source = Entity::PLACEHOLDER;
        let player = players::PlayerId(Entity::from_raw_u32(2).unwrap());
        let event = AssignCombatDamage {
            target,
            source,
            amount: 3,
            player,
        };
        assert_eq!(event.subject(), target);
        assert_eq!(event.other(), Some(source));
        assert_eq!(event.player(), player);
        assert_eq!(
            AssignCombatDamage::ID,
            rules::EventId("510.1c/assign-combat-damage")
        );
        assert_eq!(event.field(rules::Field::Amount), Some(3));
        let restrictions = AssignCombatDamage::restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, super::ONLY_BLOCKERS);
        assert_eq!(
            restrictions[0].text,
            "A blocked creature assigns its combat damage to the creatures blocking it"
        );
        assert_eq!(
            restrictions[0].condition,
            rules::points_to::<cards::Blocking>(rules::Role::Other)
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
