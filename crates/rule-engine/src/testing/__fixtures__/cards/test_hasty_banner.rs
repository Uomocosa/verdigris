use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::statics;
use crate::testing;

pub const TEST_HASTY_BANNER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Hasty Banner"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Hasty Banner", cost![1, R]);
            statics::all(CREATURE.controlled_by(YOU)).gain(keywords::haste()),
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

    use super::TEST_HASTY_BANNER;

    #[test]
    fn test_usage() {
        let banner = (TEST_HASTY_BANNER.definition)();
        assert_eq!(*banner.name, "Test Hasty Banner");
        assert_eq!(banner.types, vec![cards::CardType::Enchantment]);
        assert_eq!(banner.cost, cost![1, R]);
        assert_eq!(banner.colors(), vec![cards::Color::Red]);
        assert_eq!(
            banner.static_abilities,
            [abilities::Static {
                filter: CREATURE.controlled_by(YOU),
                change: effects::Change::Characteristic(effects::Layered::GainKeyword(
                    keywords::Keyword::Haste,
                )),
            }]
        );
    }
}
