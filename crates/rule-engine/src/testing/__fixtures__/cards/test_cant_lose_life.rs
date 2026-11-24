use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_CANT_LOSE_LIFE: testing::Card = testing::Card {
    name: testing::TestCardName("Test Can't Lose Life"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Can't Lose Life", cost![2]);
            statics::all(PLAYER.controlled_by(YOU)).cant::<rules::LoseLife>(rules::Role::Player),
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
    use crate::rules::traits::event::Event;

    use super::TEST_CANT_LOSE_LIFE;

    #[test]
    fn test_usage() {
        let cant = (TEST_CANT_LOSE_LIFE.definition)();
        assert_eq!(*cant.name, "Test Can't Lose Life");
        assert_eq!(cant.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(cant.cost, cost![2]);
        assert_eq!(
            cant.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(rules::LoseLife::ID),
                    change: rules::Change::Cant {
                        role: rules::Role::Player,
                    },
                }),
            },]
        );
    }
}
