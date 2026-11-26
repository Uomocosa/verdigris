use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::events;
use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const ONLY_BLOCKERS: rules::ClauseId<events::AssignCombatDamage> =
    rules::ClauseId::new("510.1c/only-blockers");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssignCombatDamage {
    pub target: Entity,
    pub source: Entity,
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

    #[atomic_delegate(AssignCombatDamage, Event)]
    fn clauses() -> Vec<rules::Clause> {}
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
            player,
        };
        assert_eq!(event.subject(), target);
        assert_eq!(event.other(), Some(source));
        assert_eq!(event.player(), player);
        assert_eq!(
            AssignCombatDamage::ID,
            rules::EventId("510.1c/assign-combat-damage")
        );
        let clauses = AssignCombatDamage::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, super::ONLY_BLOCKERS.key);
        assert_eq!(
            clauses[0].text,
            "A blocked creature assigns its combat damage to the creatures blocking it"
        );
        assert_eq!(clauses[0].role, rules::Role::Other);
        assert_eq!(
            clauses[0].condition,
            rules::points_to::<cards::Blocking>(rules::Role::Other)
        );
    }

    #[test]
    fn clause_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for clause in AssignCombatDamage::clauses() {
            assert!(
                RULES_TEXT.contains(clause.text),
                "clause {:?} text not found verbatim in Comprehensive Rules: {:?}",
                clause.key,
                clause.text
            );
        }
    }
}
