use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const GIANT_SPIDER: testing::Card = testing::Card {
    name: testing::TestCardName("Giant Spider"),
    definition: || {
        card! {
            cards::creature("Giant Spider", cost![3, G], pt!(2, 4));
            cards::reach(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::GIANT_SPIDER;

    #[test]
    fn test_usage() {
        let spider = (GIANT_SPIDER.definition)();
        assert_eq!(*spider.name, "Giant Spider");
        assert_eq!(spider.types, vec![cards::CardType::Creature]);
        assert_eq!(spider.cost, cost![3, G]);
        assert_eq!(spider.power_toughness, Some(pt!(2, 4)));
        assert_eq!(spider.keywords, vec![cards::Keyword::Reach]);
    }
}
