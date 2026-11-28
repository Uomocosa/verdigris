use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

pub const NONZERO_DAMAGE: rules::RestrictionId = rules::RestrictionId("120.8/nonzero");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct DealDamage {
    #[arg]
    pub source: Entity,
    #[arg]
    pub recipient: Entity,
    #[arg]
    pub amount: u32,
}

impl Event for DealDamage {
    const ID: rules::EventId = rules::EventId("120.3/deal-damage");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::SOURCE);

    #[rustfmt::skip]
    fn affected(&self) -> Vec<Entity> { vec![self.recipient] }

    #[atomic_delegate(DealDamage, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::DealDamage;

    #[test]
    fn test_usage() {
        let source = Entity::PLACEHOLDER;
        let recipient = Entity::from_raw_u32(1).unwrap();
        let event = DealDamage {
            source,
            recipient,
            amount: 3,
        };
        assert_eq!(
            event.arg(DealDamage::RECIPIENT),
            Some(rules::ArgValue::Entity(recipient))
        );
        assert_eq!(DealDamage::ID, rules::EventId("120.3/deal-damage"));
        assert_eq!(DealDamage::HOLDER, rules::ArgRef::Arg(DealDamage::SOURCE));
        let restrictions = DealDamage::restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(restrictions[0].id, super::NONZERO_DAMAGE);
        assert_eq!(
            restrictions[0].condition,
            DealDamage::AMOUNT.is(rules::Value(1).or_more())
        );
        assert_eq!(
            event.arg(DealDamage::SOURCE),
            Some(rules::ArgValue::Entity(source))
        );
        assert_eq!(
            event.arg(DealDamage::AMOUNT),
            Some(rules::ArgValue::Amount(3))
        );
        assert_eq!(
            DealDamage::args(),
            &[
                DealDamage::SOURCE,
                DealDamage::RECIPIENT,
                DealDamage::AMOUNT,
            ]
        );
    }

    #[test]
    fn restriction_rule_numbers_occur_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../docs/MagicCompRules-20250919.txt");
        for restriction in DealDamage::restrictions() {
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
