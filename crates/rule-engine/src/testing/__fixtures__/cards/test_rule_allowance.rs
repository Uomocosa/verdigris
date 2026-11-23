use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_RULE_ALLOWANCE: testing::Card = testing::Card {
    name: testing::TestCardName("Test Rule Allowance"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Rule Allowance", cost![2, U]);
            statics::all(CREATURE).allow(testing::UNTAPPED, rules::has::<cards::Flying>()),
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
    use crate::rules;
    use crate::testing;

    use super::TEST_RULE_ALLOWANCE;

    #[test]
    fn test_usage() {
        let allowance = (TEST_RULE_ALLOWANCE.definition)();
        assert_eq!(*allowance.name, "Test Rule Allowance");
        assert_eq!(
            allowance.type_line.types,
            vec![cards::CardType::Enchantment]
        );
        assert_eq!(allowance.cost, cost![2, U]);
        assert_eq!(allowance.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            allowance.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: CREATURE,
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Clause(testing::UNTAPPED.key),
                    change: rules::Change::Allow(rules::has::<cards::Flying>()),
                }),
            }]
        );
    }
}
