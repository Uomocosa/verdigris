use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const WIND_DRAKE: testing::Card = testing::Card {
    name: testing::TestCardName("Wind Drake"),
    definition: || {
        card! {
            cards::creature("Wind Drake", cost![2, U], pt!(2, 2));
            keywords::flying(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::WIND_DRAKE;

    #[test]
    fn test_usage() {
        let drake = (WIND_DRAKE.definition)();
        assert_eq!(*drake.name, "Wind Drake");
        assert_eq!(drake.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(drake.cost, cost![2, U]);
        assert_eq!(drake.power_toughness, Some(pt!(2, 2)));
        assert_eq!(
            drake.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Flying]
        );
    }
}
