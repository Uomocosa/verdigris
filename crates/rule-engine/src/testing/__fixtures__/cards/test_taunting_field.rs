use crate::CREATURE;
use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::statics;
use crate::testing;

pub const TEST_TAUNTING_FIELD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Taunting Field"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Taunting Field", cost![1, G]);
            statics::all(CREATURE.controlled_by(OPPONENT)).must_block(),
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

    use super::TEST_TAUNTING_FIELD;

    #[test]
    fn test_usage() {
        let field = (TEST_TAUNTING_FIELD.definition)();
        assert_eq!(*field.name, "Test Taunting Field");
        assert_eq!(field.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(field.cost, cost![1, G]);
        assert_eq!(field.colors(), vec![cards::Color::Green]);
        assert_eq!(
            field.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: CREATURE.controlled_by(OPPONENT),
                modification: effects::Modification::Rule(rules::RuleChange::Requirement(
                    rules::DeclareBlocker::ID,
                )),
            }]
        );
    }
}
