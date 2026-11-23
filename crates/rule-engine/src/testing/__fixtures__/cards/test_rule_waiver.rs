use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::statics;
use crate::testing;

pub const TEST_RULE_WAIVER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Rule Waiver"),
    definition: || {
        card! {
            cards::creature("Test Rule Waiver", cost![1], pt!(1, 1));
            statics::this().waive(testing::UNTAPPED),
            statics::this().demand::<testing::TestRule>(),
            statics::this().skip(testing::TEST_STEP),
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
    use crate::rules::traits::legality_rule::LegalityRule;
    use crate::testing;

    use super::TEST_RULE_WAIVER;

    #[test]
    fn test_usage() {
        let waiver = (TEST_RULE_WAIVER.definition)();
        assert_eq!(*waiver.name, "Test Rule Waiver");
        assert_eq!(waiver.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(waiver.cost, cost![1]);
        assert_eq!(waiver.power_toughness, Some(pt!(1, 1)));
        assert_eq!(
            waiver.continuous_abilities().cloned().collect::<Vec<_>>(),
            [
                abilities::Continuous {
                    filter: effects::Filter {
                        object: Some(effects::ObjectRelation::ThisObject),
                        ..CREATURE
                    },
                    change: effects::Change::Waive(testing::UNTAPPED.key),
                },
                abilities::Continuous {
                    filter: effects::Filter {
                        object: Some(effects::ObjectRelation::ThisObject),
                        ..CREATURE
                    },
                    change: effects::Change::Demand(testing::TestRule::ID),
                },
                abilities::Continuous {
                    filter: effects::Filter {
                        object: Some(effects::ObjectRelation::ThisObject),
                        ..CREATURE
                    },
                    change: effects::Change::Skip(testing::TEST_STEP),
                },
            ]
        );
    }
}
