use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_LOW_SKIMMER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Low Skimmer"),
    definition: || {
        card! {
            cards::creature("Test Low Skimmer", cost![1, U], pt!(1, 1));
            keywords::flying(),
            statics::this().waive(rules::declare_blocker::FLYING),
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

    use super::TEST_LOW_SKIMMER;

    #[test]
    fn test_usage() {
        let skimmer = (TEST_LOW_SKIMMER.definition)();
        assert_eq!(*skimmer.name, "Test Low Skimmer");
        assert_eq!(skimmer.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(skimmer.cost, cost![1, U]);
        assert_eq!(skimmer.power_toughness, Some(pt!(1, 1)));
        assert_eq!(
            skimmer.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Flying]
        );
        assert_eq!(
            skimmer.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: effects::Filter {
                    object: Some(effects::ObjectRelation::ThisObject),
                    ..CREATURE
                },
                change: effects::Change::Waive(rules::declare_blocker::FLYING.key),
            }]
        );
    }
}
