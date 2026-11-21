use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::testing;

pub const GARRUKS_COMPANION: testing::Card = testing::Card {
    name: testing::TestCardName("Garruk's Companion"),
    definition: || {
        card! {
            cards::creature("Garruk's Companion", cost![G, G], pt!(3, 2));
            cards::trample(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::GARRUKS_COMPANION;

    #[test]
    fn test_usage() {
        let companion = (GARRUKS_COMPANION.definition)();
        assert_eq!(*companion.name, "Garruk's Companion");
        assert_eq!(companion.types, vec![cards::CardType::Creature]);
        assert_eq!(companion.cost, cost![G, G]);
        assert_eq!(companion.power_toughness, Some(pt!(3, 2)));
        assert_eq!(companion.keywords, vec![cards::Keyword::Trample]);
    }
}
