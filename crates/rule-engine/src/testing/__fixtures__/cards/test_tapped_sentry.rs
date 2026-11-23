use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_TAPPED_SENTRY: testing::Card = testing::Card {
    name: testing::TestCardName("Test Tapped Sentry"),
    definition: || {
        card! {
            cards::creature("Test Tapped Sentry", cost![1, W], pt!(1, 4));
            statics::this().as_though(rules::declare_blocker::UNTAPPED),
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

    use super::TEST_TAPPED_SENTRY;

    #[test]
    fn test_usage() {
        let sentry = (TEST_TAPPED_SENTRY.definition)();
        assert_eq!(*sentry.name, "Test Tapped Sentry");
        assert_eq!(sentry.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(sentry.cost, cost![1, W]);
        assert_eq!(sentry.power_toughness, Some(pt!(1, 4)));
        assert_eq!(
            sentry.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: effects::Filter {
                    object: Some(effects::ObjectRelation::ThisObject),
                    ..CREATURE
                },
                modification: effects::Modification::Rule(rules::RuleChange::AsThough(
                    rules::declare_blocker::UNTAPPED.key,
                )),
            }]
        );
    }
}
