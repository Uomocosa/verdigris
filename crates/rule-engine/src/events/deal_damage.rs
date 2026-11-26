use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::events;
use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const NONZERO_DAMAGE: rules::ClauseId<events::DealDamage> =
    rules::ClauseId::new("120.8/nonzero");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DealDamage {
    pub source: Entity,
    pub recipient: Entity,
    pub amount: u32,
    pub controller: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for DealDamage {
    fn subject(&self) -> Entity { self.recipient }
    fn other(&self) -> Option<Entity> { Some(self.source) }
    fn player(&self) -> players::PlayerId { self.controller }
}

impl Event for DealDamage {
    const ID: rules::EventId = rules::EventId("120.3");

    fn field(&self, field: rules::Field) -> Option<u32> {
        match field {
            rules::Field::Amount => Some(self.amount),
        }
    }

    #[rustfmt::skip]
    fn affected(&self) -> Vec<Entity> { vec![self.recipient] }

    #[atomic_delegate(DealDamage, Event)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;

    use super::DealDamage;
    use crate::players;

    #[test]
    fn test_usage() {
        let source = Entity::PLACEHOLDER;
        let recipient = Entity::from_raw_u32(1).unwrap();
        let controller = players::PlayerId(Entity::from_raw_u32(2).unwrap());
        let event = DealDamage {
            source,
            recipient,
            amount: 3,
            controller,
        };
        assert_eq!(event.subject(), recipient);
        assert_eq!(event.other(), Some(source));
        assert_eq!(event.player(), controller);
        assert_eq!(DealDamage::ID, rules::EventId("120.3"));
        let clauses = DealDamage::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, super::NONZERO_DAMAGE.key);
        assert_eq!(
            clauses[0].text,
            "If a source would deal 0 damage, it does not deal damage at all."
        );
        assert_eq!(clauses[0].role, rules::Role::Subject);
        assert_eq!(
            clauses[0].condition,
            rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 1)
        );
        assert_eq!(event.field(rules::Field::Amount), Some(3));
    }

    #[test]
    fn clause_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for clause in DealDamage::clauses() {
            assert!(
                RULES_TEXT.contains(clause.text),
                "clause {:?} text not found verbatim in Comprehensive Rules: {:?}",
                clause.key,
                clause.text
            );
        }
    }
}
