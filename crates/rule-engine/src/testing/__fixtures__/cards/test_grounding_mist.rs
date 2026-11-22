use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_GROUNDING_MIST: testing::Card = testing::Card {
    name: testing::TestCardName("Test Grounding Mist"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Grounding Mist", cost![1, G]);
            statics::all(CREATURE.controlled_by(OPPONENT)).lose(keywords::flying()),
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

    use super::TEST_GROUNDING_MIST;

    #[test]
    fn test_usage() {
        let mist = (TEST_GROUNDING_MIST.definition)();
        assert_eq!(*mist.name, "Test Grounding Mist");
        assert_eq!(mist.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(mist.cost, cost![1, G]);
        assert_eq!(mist.colors(), vec![cards::Color::Green]);
        assert_eq!(
            mist.static_abilities,
            [abilities::Continuous {
                filter: CREATURE.controlled_by(OPPONENT),
                change: effects::Change::Characteristic(effects::Layered::LoseKeyword(
                    keywords::StaticKeyword::Flying,
                )),
            }]
        );
    }
}
