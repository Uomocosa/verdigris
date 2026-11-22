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
        assert_eq!(tundra.type_line.supertypes, Vec::<cards::Supertype>::new());
        assert_eq!(tundra.type_line.types, vec![cards::CardType::Land]);
        assert_eq!(
            tundra.type_line.land_types().collect::<Vec<_>>(),
            vec![cards::LandType::Plains, cards::LandType::Island]
        );
    }
}
