use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_WEIGHT_OF_COMMAND: testing::Card = testing::Card {
    name: testing::TestCardName("Test Weight of Command"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Weight of Command", cost![2, U]);
            statics::all(CREATURE.controlled_by(OPPONENT)).tap_when_attacking(),
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

    use super::TEST_WEIGHT_OF_COMMAND;

    #[test]
    fn test_usage() {
        let weight = (TEST_WEIGHT_OF_COMMAND.definition)();
        assert_eq!(*weight.name, "Test Weight of Command");
        assert_eq!(weight.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(weight.cost, cost![2, U]);
        assert_eq!(weight.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            weight.static_abilities,
            [abilities::Static {
                filter: CREATURE.controlled_by(OPPONENT),
                change: effects::Change::Rule {
                    question: rules::Question::TapsWhenAttacking,
                    answer: rules::Answer::TapsWhenAttacking(true),
                },
            }]
        );
    }
}
