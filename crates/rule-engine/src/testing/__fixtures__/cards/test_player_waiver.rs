use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_PLAYER_WAIVER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Player Waiver"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Player Waiver", cost![2, U]);
            statics::all(PLAYER.controlled_by(YOU)).as_though(testing::UNTAPPED),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;
    use crate::testing;

    use super::TEST_PLAYER_WAIVER;

    #[test]
    fn test_usage() {
        let waiver = (TEST_PLAYER_WAIVER.definition)();
        assert_eq!(*waiver.name, "Test Player Waiver");
        assert_eq!(waiver.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(waiver.cost, cost![2, U]);
        assert_eq!(waiver.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            waiver.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange::AsThough(
                    testing::UNTAPPED.key,
                )),
            }]
        );
    }
}
