use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const IRON_GOLEM: testing::Card = testing::Card {
    name: testing::TestCardName("Iron Golem"),
    definition: || {
        card! {
            cards::artifact_creature("Iron Golem", cost![4], pt!(5, 3));
            keywords::vigilance(),
            keywords::must_attack_if_able(),
            keywords::must_block_if_able(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::IRON_GOLEM;

    #[test]
    fn test_usage() {
        let golem = (IRON_GOLEM.definition)();
        assert_eq!(*golem.name, "Iron Golem");
        assert_eq!(
            golem.type_line.types,
            vec![cards::CardType::Artifact, cards::CardType::Creature]
        );
        assert_eq!(golem.cost, cost![4]);
        assert_eq!(golem.power_toughness, Some(pt!(5, 3)));
        assert_eq!(
            golem.keywords,
            vec![
                keywords::Keyword::Vigilance,
                keywords::Keyword::MustAttackIfAble,
                keywords::Keyword::MustBlockIfAble,
            ]
        );
    }
}
