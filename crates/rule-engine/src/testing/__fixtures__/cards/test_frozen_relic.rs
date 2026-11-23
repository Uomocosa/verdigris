use crate::UNTAP_STEP;
use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::statics;
use crate::testing;

pub const TEST_FROZEN_RELIC: testing::Card = testing::Card {
    name: testing::TestCardName("Test Frozen Relic"),
    definition: || {
        card! {
            cards::creature("Test Frozen Relic", cost![2], pt!(1, 1));
            statics::this().skip(UNTAP_STEP),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::UNTAP_STEP;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;

    use super::TEST_FROZEN_RELIC;

    #[test]
    fn test_usage() {
        let relic = (TEST_FROZEN_RELIC.definition)();
        assert_eq!(*relic.name, "Test Frozen Relic");
        assert_eq!(relic.type_line.types, vec![cards::CardType::Creature]);
        assert_eq!(relic.cost, cost![2]);
        assert_eq!(relic.power_toughness, Some(pt!(1, 1)));
        assert_eq!(
            relic.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: effects::Filter {
                    object: Some(effects::ObjectRelation::ThisObject),
                    ..CREATURE
                },
                change: effects::Change::Skip(UNTAP_STEP),
            }]
        );
    }
}
