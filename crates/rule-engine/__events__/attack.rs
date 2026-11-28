use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::rules;
use crate::rules::traits::event::Event;

pub const CREATURE: rules::RestrictionId = rules::RestrictionId("508.1a/creature");
pub const CONTROLS: rules::RestrictionId = rules::RestrictionId("508.1a/controls");
pub const UNTAPPED: rules::RestrictionId = rules::RestrictionId("508.1a/untapped");
pub const SUMMONING_SICK: rules::RestrictionId =
    rules::RestrictionId("508.1a/controlled-since-turn-began");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct Attack {
    #[arg]
    pub attacker: Entity,
}

impl Event for Attack {
    const ID: rules::EventId = rules::EventId("508.1a/attack");
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(Self::ATTACKER);

    #[atomic_delegate(Attack, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::effects;
    use crate::events;
    use crate::keywords;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::Attack;
    use super::CONTROLS;
    use super::CREATURE;
    use super::SUMMONING_SICK;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(Attack::ID, rules::EventId("508.1a/attack"));
        assert_eq!(Attack::HOLDER, rules::ArgRef::Arg(Attack::ATTACKER));
        let restrictions = Attack::restrictions();
        assert_eq!(restrictions.len(), 4);
        assert_eq!(restrictions[0].id, CREATURE);
        assert_eq!(
            restrictions[0].condition,
            rules::has::<cards::Creature>().of(Attack::ATTACKER)
        );
        assert_eq!(restrictions[1].id, CONTROLS);
        assert_eq!(
            restrictions[1].condition,
            rules::has::<cards::OnBattlefield>().of(Attack::ATTACKER)
        );
        assert_eq!(restrictions[2].id, UNTAPPED);
        assert_eq!(
            restrictions[2].condition,
            (!rules::has::<cards::Tapped>()).of(Attack::ATTACKER)
        );
        assert_eq!(restrictions[3].id, SUMMONING_SICK);
        assert_eq!(
            restrictions[3].condition,
            (!rules::has::<cards::SummoningSick>()).of(Attack::ATTACKER)
        );
        assert_eq!(Attack::args(), &[Attack::ATTACKER,]);
        let event = Attack {
            attacker: Entity::PLACEHOLDER,
        };
        assert_eq!(
            event.arg(Attack::ATTACKER),
            Some(rules::ArgValue::Entity(Entity::PLACEHOLDER))
        );
    }

    #[test]
    fn restriction_rule_numbers_occur_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../docs/MagicCompRules-20250919.txt");
        let keyword_restrictions = [
            keywords::deathtouch().abilities(),
            keywords::defender().abilities(),
            keywords::double_strike().abilities(),
            keywords::first_strike().abilities(),
            keywords::flying().abilities(),
            keywords::haste().abilities(),
            keywords::hexproof().abilities(),
            keywords::indestructible().abilities(),
            keywords::infect().abilities(),
            keywords::lifelink().abilities(),
            keywords::reach().abilities(),
            keywords::trample().abilities(),
            keywords::vigilance().abilities(),
        ]
        .into_iter()
        .flatten()
        .filter_map(|modification| match modification {
            effects::Modification::Rule(change) => match change.change {
                rules::Change::Restrict { restriction, .. } => Some(restriction),
                _ => None,
            },
            effects::Modification::Characteristic(_) => None,
        });
        for restriction in events::DeclareBlocker::restrictions()
            .into_iter()
            .chain(Attack::restrictions())
            .chain(keyword_restrictions)
        {
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
