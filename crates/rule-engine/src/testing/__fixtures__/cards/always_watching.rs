use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const ALWAYS_WATCHING: testing::Card = testing::Card {
    name: testing::TestCardName("Always Watching"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Always Watching", cost![1, W, W]);
            statics::all(CREATURE.controlled_by(YOU).nontoken()).get_pt(1, 1),
            statics::all(CREATURE.controlled_by(YOU).nontoken()).gain(keywords::vigilance()),
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
    use crate::keywords;

    use super::ALWAYS_WATCHING;

    #[test]
    fn test_usage() {
        let watching = (ALWAYS_WATCHING.definition)();
        assert_eq!(*watching.name, "Always Watching");
        assert_eq!(watching.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(watching.cost, cost![1, W, W]);
        assert_eq!(watching.colors(), vec![cards::Color::White]);
        assert_eq!(
            watching.continuous_abilities().cloned().collect::<Vec<_>>(),
            [
                abilities::Continuous {
                    filter: CREATURE.controlled_by(YOU).nontoken(),
                    change: effects::Change::Characteristic(effects::Layered::ModifyPT {
                        power: 1,
                        toughness: 1,
                    }),
                },
                abilities::Continuous {
                    filter: CREATURE.controlled_by(YOU).nontoken(),
                    change: effects::Change::Characteristic(effects::Layered::GainKeyword(
                        keywords::StaticKeyword::Vigilance,
                    )),
                },
            ]
        );
    }
}
