use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const FENCING_ACE: testing::Card = testing::Card {
    name: testing::TestCardName("Fencing Ace"),
    definition: || {
        card! {
            cards::creature("Fencing Ace", cost![1, W], pt!(1, 1));
            keywords::double_strike(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::FENCING_ACE;

    #[test]
    fn test_usage() {
        let ace = (FENCING_ACE.definition)();
        assert_eq!(*ace.name, "Fencing Ace");
        assert_eq!(ace.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(ace.cost, cost![1, W]);
        assert_eq!(ace.power_toughness, Some(pt!(1, 1)));
        assert_eq!(ace.keywords, vec![keywords::Keyword::DoubleStrike]);
    }
}
