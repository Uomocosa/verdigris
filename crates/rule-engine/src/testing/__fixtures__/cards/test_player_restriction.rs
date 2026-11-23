use crate::PLAYER;
use crate::YOU;
use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;
use crate::testing;

pub const TEST_PLAYER_RESTRICTION: testing::Card = testing::Card {
    name: testing::TestCardName("Test Player Restriction"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Player Restriction", cost![2]);
            abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(testing::TestRule::ID),
                    change: rules::Change::OnlyIf {
                        role: rules::Role::Player,
                        condition: !rules::Predicate::True,
                    },
                }),
            },
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;
    use crate::testing;

    use super::TEST_PLAYER_RESTRICTION;

    #[test]
    fn test_usage() {
        let restriction = (TEST_PLAYER_RESTRICTION.definition)();
        assert_eq!(*restriction.name, "Test Player Restriction");
        assert_eq!(
            restriction.type_line.types,
            vec![cards::CardType::Enchantment]
        );
        assert_eq!(restriction.cost, cost![2]);
        assert_eq!(
            restriction.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(testing::TestRule::ID),
                    change: rules::Change::OnlyIf {
                        role: rules::Role::Player,
                        condition: !rules::Predicate::True,
                    },
                }),
            }]
        );
    }
}
