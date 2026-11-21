use crate::cards;
use crate::testing;

pub const MOUNTAIN: testing::Card = testing::Card {
    name: testing::TestCardName("Mountain"),
    definition: || {
        cards::land("Mountain", [cards::LandType::Mountain]).supertype(cards::Supertype::Basic)
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::MOUNTAIN;

    #[test]
    fn test_usage() {
        let mountain = (MOUNTAIN.definition)();
        assert_eq!(*mountain.name, "Mountain");
        assert_eq!(mountain.supertypes, vec![cards::Supertype::Basic]);
        assert_eq!(mountain.types, vec![cards::CardType::Land]);
        assert_eq!(mountain.land_types, vec![cards::LandType::Mountain]);
    }
}
