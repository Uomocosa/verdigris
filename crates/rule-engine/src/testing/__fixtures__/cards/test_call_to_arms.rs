use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_CALL_TO_ARMS: testing::Card = testing::Card {
    name: testing::TestCardName("Test Call to Arms"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Call to Arms", cost![1, R]);
            statics::all(CREATURE.controlled_by(OPPONENT)).must_attack(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::OPPONENT;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::TEST_CALL_TO_ARMS;

    #[test]
    fn test_usage() {
        let arms = (TEST_CALL_TO_ARMS.definition)();
        assert_eq!(*arms.name, "Test Call to Arms");
        assert_eq!(arms.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(arms.cost, cost![1, R]);
        assert_eq!(arms.colors(), vec![cards::Color::Red]);
        assert_eq!(
            arms.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: CREATURE.controlled_by(OPPONENT),
                change: effects::Change::Demand(rules::DeclareAttacker::ID),
            }]
        );
    }
}
