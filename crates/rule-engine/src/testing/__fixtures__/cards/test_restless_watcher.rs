use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_RESTLESS_WATCHER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Restless Watcher"),
    definition: || {
        card! {
            cards::creature("Test Restless Watcher", cost![2, U], pt!(2, 2));
            statics::this().as_though(rules::declare_attacker::UNTAPPED),
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
    use crate::pt;
    use crate::rules;

    use super::TEST_RESTLESS_WATCHER;

    #[test]
    fn test_usage() {
        let watcher = (TEST_RESTLESS_WATCHER.definition)();
        assert_eq!(*watcher.name, "Test Restless Watcher");
        assert_eq!(watcher.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(watcher.cost, cost![2, U]);
        assert_eq!(watcher.power_toughness, Some(pt!(2, 2)));
        assert_eq!(
            watcher.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: effects::Filter {
                    object: Some(effects::ObjectRelation::ThisObject),
                    ..CREATURE
                },
                modification: effects::Modification::Rule(rules::RuleChange::AsThough(
                    rules::declare_attacker::UNTAPPED.key
                )),
            }]
        );
    }
}
