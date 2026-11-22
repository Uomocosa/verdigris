use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const ISAMARU_HOUND_OF_KONDA: testing::Card = testing::Card {
    name: testing::TestCardName("Isamaru, Hound of Konda"),
    definition: || {
        cards::creature("Isamaru, Hound of Konda", cost![W], pt!(2, 2))
            .supertype(cards::Supertype::Legendary)
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::ISAMARU_HOUND_OF_KONDA;

    #[test]
    fn test_usage() {
        let isamaru = (ISAMARU_HOUND_OF_KONDA.definition)();
        assert_eq!(*isamaru.name, "Isamaru, Hound of Konda");
        assert_eq!(
            isamaru.type_line.supertypes,
            vec![cards::Supertype::Legendary]
        );
        assert_eq!(isamaru.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(isamaru.cost, cost![W]);
        assert_eq!(isamaru.power_toughness, Some(pt!(2, 2)));
    }
}
