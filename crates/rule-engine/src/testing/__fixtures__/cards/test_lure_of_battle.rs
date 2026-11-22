use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_LURE_OF_BATTLE: testing::Card = testing::Card {
    name: testing::TestCardName("Test Lure of Battle"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Lure of Battle", cost![1, G]);
            statics::all(CREATURE.controlled_by(OPPONENT)).must_block(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::OPPONENT;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;

    use super::TEST_LURE_OF_BATTLE;

    #[test]
    fn test_usage() {
        let lure = (TEST_LURE_OF_BATTLE.definition)();
        assert_eq!(*lure.name, "Test Lure of Battle");
        assert_eq!(lure.types, vec![cards::CardType::Enchantment]);
        assert_eq!(lure.cost, cost![1, G]);
        assert_eq!(lure.colors(), vec![cards::Color::Green]);
        assert_eq!(
            lure.static_abilities,
            [abilities::Static {
                filter: CREATURE.controlled_by(OPPONENT),
                change: effects::Change::Rule {
                    question: rules::Question::MustBlock,
                    answer: rules::Answer::MustBlock(true),
                },
            }]
        );
    }
}
