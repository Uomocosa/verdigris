use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const SPORE_CRAWLER: testing::Card = testing::Card {
    name: testing::TestCardName("Spore Crawler"),
    definition: || {
        card! {
            cards::creature("Spore Crawler", cost![2, G], pt!(3, 2));
            abilities::when(abilities::when_this_dies(), effects::draw(1)),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;

    use super::SPORE_CRAWLER;

    #[test]
    fn test_usage() {
        let crawler = (SPORE_CRAWLER.definition)();
        assert_eq!(*crawler.name, "Spore Crawler");
        assert_eq!(crawler.types, vec![cards::CardType::Creature]);
        assert_eq!(crawler.cost, cost![2, G]);
        assert_eq!(crawler.power_toughness, Some(pt!(3, 2)));
        assert_eq!(
            crawler.triggered_abilities,
            [abilities::TriggeredAbility {
                event: abilities::when_this_dies(),
                intervening_if: None,
                effect: effects::draw(1),
            }]
        );
    }
}
