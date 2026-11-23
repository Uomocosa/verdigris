use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_RULE_RESTRICTION: testing::Card = testing::Card {
    name: testing::TestCardName("Test Rule Restriction"),
    definition: || {
        card! {
            cards::creature("Test Rule Restriction", cost![2], pt!(1, 1));
            statics::this().restriction::<testing::RestrictedRule>(rules::at_least(rules::Characteristic::Power, 3)),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;
    use crate::testing;

    use super::TEST_RULE_RESTRICTION;

    #[test]
    fn test_usage() {
        let restriction = (TEST_RULE_RESTRICTION.definition)();
        assert_eq!(*restriction.name, "Test Rule Restriction");
        assert_eq!(restriction.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(restriction.cost, cost![2]);
        assert_eq!(restriction.power_toughness, Some(pt!(1, 1)));
        assert_eq!(
            restriction.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: effects::Filter {
                    object: Some(effects::ObjectRelation::ThisObject),
                    ..CREATURE
                },
                modification: effects::Modification::Restriction(
                    testing::RestrictedRule::ID,
                    rules::at_least(rules::Characteristic::Power, 3),
                ),
            }]
        );
    }
}
