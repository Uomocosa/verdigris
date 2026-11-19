use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const GRIZZLY_BEARS: testing::Card = testing::Card {
    name: testing::TestCardName("Grizzly Bears"),
    definition: || cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2)),
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::pt;

    use super::GRIZZLY_BEARS;

    #[test]
    fn test_usage() {
        let bears = (GRIZZLY_BEARS.definition)();
        assert_eq!(*bears.name, "Grizzly Bears");
        assert_eq!(bears.types, vec![cards::CardType::Creature]);
        assert_eq!(bears.power_toughness, Some(pt!(2, 2)));
    }
}
