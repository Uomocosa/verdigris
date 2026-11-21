use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const WATCHWOLF: testing::Card = testing::Card {
    name: testing::TestCardName("Watchwolf"),
    definition: || cards::creature("Watchwolf", cost![G, W], pt!(3, 3)),
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::WATCHWOLF;

    #[test]
    fn test_usage() {
        let wolf = (WATCHWOLF.definition)();
        assert_eq!(*wolf.name, "Watchwolf");
        assert_eq!(wolf.types, vec![cards::CardType::Creature]);
        assert_eq!(wolf.cost, cost![G, W]);
        assert_eq!(wolf.power_toughness, Some(pt!(3, 3)));
    }
}
