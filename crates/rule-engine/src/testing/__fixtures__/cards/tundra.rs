use crate::cards;
use crate::testing;

pub const TUNDRA: testing::Card = testing::Card {
    name: testing::TestCardName("Tundra"),
    definition: || cards::land("Tundra", [cards::LandType::Plains, cards::LandType::Island]),
};

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::TUNDRA;

    #[test]
    fn test_usage() {
        let tundra = (TUNDRA.definition)();
        assert_eq!(*tundra.name, "Tundra");
        assert_eq!(tundra.supertypes, Vec::<cards::Supertype>::new());
        assert_eq!(tundra.types, vec![cards::CardType::Land]);
        assert_eq!(
            tundra.land_types,
            vec![cards::LandType::Plains, cards::LandType::Island]
        );
    }
}
