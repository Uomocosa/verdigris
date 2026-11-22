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
        assert_eq!(mountain.type_line.supertypes, vec![cards::Supertype::Basic]);
        assert_eq!(mountain.type_line.types, vec![cards::CardType::Land]);
        assert_eq!(
            mountain.type_line.land_types().collect::<Vec<_>>(),
            vec![cards::LandType::Mountain]
        );
    }
}
