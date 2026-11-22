use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const GLADECOVER_SCOUT: testing::Card = testing::Card {
    name: testing::TestCardName("Gladecover Scout"),
    definition: || {
        card! {
            cards::creature("Gladecover Scout", cost![G], pt!(1, 1));
            keywords::hexproof(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::GLADECOVER_SCOUT;

    #[test]
    fn test_usage() {
        let scout = (GLADECOVER_SCOUT.definition)();
        assert_eq!(*scout.name, "Gladecover Scout");
        assert_eq!(scout.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(scout.cost, cost![G]);
        assert_eq!(scout.power_toughness, Some(pt!(1, 1)));
        assert_eq!(scout.keywords, vec![keywords::StaticKeyword::Hexproof]);
    }
}
