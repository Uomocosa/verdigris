use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const PLAGUE_STINGER: testing::Card = testing::Card {
    name: testing::TestCardName("Plague Stinger"),
    definition: || {
        card! {
            cards::creature("Plague Stinger", cost![1, B], pt!(1, 1));
            keywords::flying(),
            keywords::infect(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::PLAGUE_STINGER;

    #[test]
    fn test_usage() {
        let stinger = (PLAGUE_STINGER.definition)();
        assert_eq!(*stinger.name, "Plague Stinger");
        assert_eq!(stinger.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(stinger.cost, cost![1, B]);
        assert_eq!(stinger.power_toughness, Some(pt!(1, 1)));
        assert_eq!(
            stinger.keywords().collect::<Vec<_>>(),
            vec![
                keywords::StaticKeyword::Flying,
                keywords::StaticKeyword::Infect
            ]
        );
    }
}
