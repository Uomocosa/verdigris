use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const WALL_OF_STONE: testing::Card = testing::Card {
    name: testing::TestCardName("Wall of Stone"),
    definition: || {
        card! {
            cards::creature("Wall of Stone", cost![1, R, R], pt!(0, 8));
            keywords::defender(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::WALL_OF_STONE;

    #[test]
    fn test_usage() {
        let wall = (WALL_OF_STONE.definition)();
        assert_eq!(*wall.name, "Wall of Stone");
        assert_eq!(wall.types, vec![cards::CardType::Creature]);
        assert_eq!(wall.cost, cost![1, R, R]);
        assert_eq!(wall.power_toughness, Some(pt!(0, 8)));
        assert_eq!(wall.keywords, vec![keywords::Keyword::Defender]);
    }
}
