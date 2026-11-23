use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_CANT_DISCARD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Cant Discard"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Cant Discard", cost![2]);
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::Discard>(),
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::GainLife>(),
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::Mill>(),
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::CreateTokens>(),
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
    use crate::rules::traits::event::Event;

    use super::TEST_CANT_DISCARD;

    #[test]
    fn test_usage() {
        let cant = (TEST_CANT_DISCARD.definition)();
        assert_eq!(*cant.name, "Test Cant Discard");
        assert_eq!(cant.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(cant.cost, cost![2]);
        assert_eq!(
            cant.continuous_abilities().cloned().collect::<Vec<_>>(),
            [
                abilities::Continuous {
                    filter: PLAYER.controlled_by(YOU),
                    modification: effects::Modification::Rule(rules::RuleChange {
                        target: rules::Target::Event(rules::Discard::ID),
                        change: rules::Change::Cant,
                    }),
                },
                abilities::Continuous {
                    filter: PLAYER.controlled_by(YOU),
                    modification: effects::Modification::Rule(rules::RuleChange {
                        target: rules::Target::Event(rules::GainLife::ID),
                        change: rules::Change::Cant,
                    }),
                },
                abilities::Continuous {
                    filter: PLAYER.controlled_by(YOU),
                    modification: effects::Modification::Rule(rules::RuleChange {
                        target: rules::Target::Event(rules::Mill::ID),
                        change: rules::Change::Cant,
                    }),
                },
                abilities::Continuous {
                    filter: PLAYER.controlled_by(YOU),
                    modification: effects::Modification::Rule(rules::RuleChange {
                        target: rules::Target::Event(rules::CreateTokens::ID),
                        change: rules::Change::Cant,
                    }),
                },
            ]
        );
    }
}
