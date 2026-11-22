use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_WALL_OF_DREAD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Wall of Dread"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Wall of Dread", cost![2, B]);
            statics::all(CREATURE.controlled_by(OPPONENT)).lose(keywords::defender()),
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
    use crate::keywords;

    use super::TEST_WALL_OF_DREAD;

    #[test]
    fn test_usage() {
        let dread = (TEST_WALL_OF_DREAD.definition)();
        assert_eq!(*dread.name, "Test Wall of Dread");
        assert_eq!(dread.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(dread.cost, cost![2, B]);
        assert_eq!(dread.colors(), vec![cards::Color::Black]);
        assert_eq!(
            dread.continuous_abilities().copied().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: CREATURE.controlled_by(OPPONENT),
                change: effects::Change::Characteristic(effects::Layered::LoseKeyword(
                    keywords::StaticKeyword::Defender,
                )),
            }]
        );
    }
}
