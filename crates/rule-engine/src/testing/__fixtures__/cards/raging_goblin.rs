use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const RAGING_GOBLIN: testing::Card = testing::Card {
    name: testing::TestCardName("Raging Goblin"),
    definition: || {
        card! {
            cards::creature("Raging Goblin", cost![R], pt!(1, 1));
            keywords::haste(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::RAGING_GOBLIN;

    #[test]
    fn test_usage() {
        let goblin = (RAGING_GOBLIN.definition)();
        assert_eq!(*goblin.name, "Raging Goblin");
        assert_eq!(goblin.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(goblin.cost, cost![R]);
        assert_eq!(goblin.power_toughness, Some(pt!(1, 1)));
        assert_eq!(
            goblin.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Haste]
        );
    }
}
