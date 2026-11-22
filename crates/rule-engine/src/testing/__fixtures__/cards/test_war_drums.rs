use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_WAR_DRUMS: testing::Card = testing::Card {
    name: testing::TestCardName("Test War Drums"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test War Drums", cost![2, R]);
            statics::all(CREATURE.controlled_by(YOU)).must_attack(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;

    use super::TEST_WAR_DRUMS;

    #[test]
    fn test_usage() {
        let drums = (TEST_WAR_DRUMS.definition)();
        assert_eq!(*drums.name, "Test War Drums");
        assert_eq!(drums.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(drums.cost, cost![2, R]);
        assert_eq!(drums.colors(), vec![cards::Color::Red]);
        assert_eq!(
            drums.continuous_abilities().copied().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: CREATURE.controlled_by(YOU),
                change: effects::Change::Rule {
                    question: rules::Question::MustAttack,
                    answer: rules::Answer::MustAttack(true),
                },
            }]
        );
    }
}
