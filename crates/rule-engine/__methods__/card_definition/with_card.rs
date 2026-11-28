use std::collections::HashSet;

use crate::abilities;
use crate::cards;
use crate::effects;
use crate::rules;

pub fn with_card(definition: cards::CardDefinition, module: &'static str) -> cards::CardDefinition {
    let card = module.rsplit("::").next().map_or(module, |card| card);
    let mut definition = definition;
    let stamped_text: Vec<abilities::Ability> =
        definition.text.drain(..).map(|ability| ability.with_card(card)).collect();
    definition.text = stamped_text;
    let mut stamped = HashSet::new();
    for ability in &definition.text {
        if let abilities::Ability::Static(abilities::StaticAbility::Continuous(continuous)) =
            ability
            && let effects::Modification::Rule(rules::RuleChange {
                change: rules::Change::Restrict { restriction, .. },
                ..
            }) = &continuous.modification
            && restriction.id.card == card
        {
            assert!(
                stamped.insert((restriction.id.kind, restriction.id.event)),
                "card {card} stamps {} on {} twice; write one RestrictionId by hand",
                restriction.id.kind,
                *restriction.id.event,
            );
        }
    }
    definition
}

#[cfg(test)]
mod tests {
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::creature;
    use crate::effects;
    use crate::events;
    use crate::pt;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;
    use crate::testing;

    use super::with_card;

    #[test]
    fn test_usage() {
        let definition = with_card(
            cards::creature("Test", cost![2], pt!(1, 1)).abilities([
                abilities::Ability::from(
                    statics::all(creature()).cant::<events::Draw>(events::Draw::PLAYER),
                ),
                abilities::Ability::from(
                    statics::all(creature()).restriction::<testing::RestrictedRule>(
                        testing::RestrictedRule::ATTACKER,
                        rules::Condition::True,
                    ),
                ),
            ]),
            "some::test_module",
        );
        let mut stamped = definition.continuous_abilities().filter_map(|continuous| {
            match &continuous.modification {
                effects::Modification::Rule(rules::RuleChange {
                    change: rules::Change::Restrict { restriction, .. },
                    ..
                }) => Some(restriction.id),
                _ => None,
            }
        });
        assert_eq!(
            stamped.next(),
            Some(rules::RestrictionId {
                card: "test_module",
                kind: "cant",
                event: events::Draw::ID,
            })
        );
        assert_eq!(
            stamped.next(),
            Some(rules::RestrictionId {
                card: "test_module",
                kind: "restriction",
                event: testing::RestrictedRule::ID,
            })
        );
        assert_eq!(stamped.next(), None);
    }
}
