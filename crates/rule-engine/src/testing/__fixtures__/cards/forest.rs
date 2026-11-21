use crate::cards;
use crate::testing;

pub const FOREST: testing::Card = testing::Card {
    name: testing::TestCardName("Forest"),
    definition: || {
        cards::land("Forest", [cards::LandType::Forest]).supertype(cards::Supertype::Basic)
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::FOREST;

    #[test]
    fn test_usage() {
        let forest = (FOREST.definition)();
        assert_eq!(*forest.name, "Forest");
        assert_eq!(forest.supertypes, vec![cards::Supertype::Basic]);
        assert_eq!(forest.types, vec![cards::CardType::Land]);
        assert_eq!(forest.land_types, vec![cards::LandType::Forest]);
    }
}
