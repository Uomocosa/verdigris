use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_EAGER_WALL: testing::Card = testing::Card {
    name: testing::TestCardName("Test Eager Wall"),
    definition: || {
        card! {
            cards::creature("Test Eager Wall", cost![1, W], pt!(0, 4));
            keywords::defender(),
            statics::this().waive(rules::DEFENDER),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::keywords;
    use crate::pt;
    use crate::rules;

    use super::TEST_EAGER_WALL;

    #[test]
    fn test_usage() {
        let wall = (TEST_EAGER_WALL.definition)();
        assert_eq!(*wall.name, "Test Eager Wall");
        assert_eq!(wall.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(wall.cost, cost![1, W]);
        assert_eq!(wall.power_toughness, Some(pt!(0, 4)));
        assert_eq!(
            wall.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Defender]
        );
        assert_eq!(
            wall.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: effects::Filter {
                    object: Some(effects::ObjectRelation::ThisObject),
                    ..CREATURE
                },
                change: effects::Change::Waive(rules::DEFENDER.key),
            }]
        );
    }
}
