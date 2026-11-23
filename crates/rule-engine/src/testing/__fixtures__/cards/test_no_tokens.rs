use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_NO_TOKENS: testing::Card = testing::Card {
    name: testing::TestCardName("Test No Tokens"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test No Tokens", cost![2]);
            statics::all(PLAYER.controlled_by(YOU))
                .cant::<rules::EnterBattlefield>(rules::Role::Player),
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

    use super::TEST_NO_TOKENS;

    #[test]
    fn test_usage() {
        let barren = (TEST_NO_TOKENS.definition)();
        assert_eq!(*barren.name, "Test No Tokens");
        assert_eq!(barren.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(barren.cost, cost![2]);
        assert_eq!(
            barren.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(rules::EnterBattlefield::ID),
                    change: rules::Change::Cant {
                        role: rules::Role::Player
                    },
                }),
            },]
        );
    }
}
