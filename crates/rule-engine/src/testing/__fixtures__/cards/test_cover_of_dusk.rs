use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_COVER_OF_DUSK: testing::Card = testing::Card {
    name: testing::TestCardName("Test Cover of Dusk"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Cover of Dusk", cost![2, W]);
            statics::all(CREATURE.controlled_by(YOU).attacking()).attack_without_tapping(),
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

    use super::TEST_COVER_OF_DUSK;

    #[test]
    fn test_usage() {
        let cover = (TEST_COVER_OF_DUSK.definition)();
        assert_eq!(*cover.name, "Test Cover of Dusk");
        assert_eq!(cover.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(cover.cost, cost![2, W]);
        assert_eq!(cover.colors(), vec![cards::Color::White]);
        assert_eq!(
            cover.static_abilities,
            [abilities::Static {
                filter: CREATURE.controlled_by(YOU).attacking(),
                change: effects::Change::Rule {
                    question: rules::Question::TapsWhenAttacking,
                    answer: rules::Answer::TapsWhenAttacking(false),
                },
            }]
        );
    }
}
