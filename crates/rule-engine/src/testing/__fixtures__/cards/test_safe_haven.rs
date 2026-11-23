use crate::CREATURE;
use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_SAFE_HAVEN: testing::Card = testing::Card {
    name: testing::TestCardName("Test Safe Haven"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Safe Haven", cost![2]);
            statics::all(CREATURE.controlled_by(YOU)).cant::<rules::Destroy>(rules::Role::Subject),
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::Counter>(rules::Role::Player),
            statics::all(CREATURE.controlled_by(YOU)).cant::<rules::DealDamage>(rules::Role::Subject),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::PLAYER;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::TEST_SAFE_HAVEN;

    #[test]
    fn test_usage() {
        let haven = (TEST_SAFE_HAVEN.definition)();
        assert_eq!(*haven.name, "Test Safe Haven");
        assert_eq!(haven.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(haven.cost, cost![2]);
        assert_eq!(
            haven.continuous_abilities().cloned().collect::<Vec<_>>(),
            [
                abilities::Continuous {
                    filter: CREATURE.controlled_by(YOU),
                    modification: effects::Modification::Rule(rules::RuleChange {
                        target: rules::Target::Event(rules::Destroy::ID),
                        change: rules::Change::Cant {
                            role: rules::Role::Subject
                        },
                    }),
                },
                abilities::Continuous {
                    filter: PLAYER.controlled_by(YOU),
                    modification: effects::Modification::Rule(rules::RuleChange {
                        target: rules::Target::Event(rules::Counter::ID),
                        change: rules::Change::Cant {
                            role: rules::Role::Player
                        },
                    }),
                },
                abilities::Continuous {
                    filter: CREATURE.controlled_by(YOU),
                    modification: effects::Modification::Rule(rules::RuleChange {
                        target: rules::Target::Event(rules::DealDamage::ID),
                        change: rules::Change::Cant {
                            role: rules::Role::Subject
                        },
                    }),
                },
            ]
        );
    }
}
