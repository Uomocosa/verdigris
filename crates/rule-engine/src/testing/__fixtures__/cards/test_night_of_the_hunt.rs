use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_NIGHT_OF_THE_HUNT: testing::Card = testing::Card {
    name: testing::TestCardName("Test Night of the Hunt"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Night of the Hunt", cost![2, B]);
            statics::all(CREATURE.controlled_by(OPPONENT)).lose(keywords::vigilance()),
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

    use super::TEST_NIGHT_OF_THE_HUNT;

    #[test]
    fn test_usage() {
        let night = (TEST_NIGHT_OF_THE_HUNT.definition)();
        assert_eq!(*night.name, "Test Night of the Hunt");
        assert_eq!(night.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(night.cost, cost![2, B]);
        assert_eq!(night.colors(), vec![cards::Color::Black]);
        assert_eq!(
            night.static_abilities,
            [abilities::Continuous {
                filter: CREATURE.controlled_by(OPPONENT),
                change: effects::Change::Characteristic(effects::Layered::LoseKeyword(
                    keywords::StaticKeyword::Vigilance,
                )),
            }]
        );
    }
}
