use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_TIMID_FIELD: testing::Card = testing::Card {
    name: testing::TestCardName("Test Timid Field"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Timid Field", cost![1, W]);
            statics::all(CREATURE)
                .restriction::<rules::DeclareBlocker>(rules::at_least(
                    rules::Characteristic::Power,
                    3,
                )),
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
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::TEST_TIMID_FIELD;

    #[test]
    fn test_usage() {
        let field = (TEST_TIMID_FIELD.definition)();
        assert_eq!(*field.name, "Test Timid Field");
        assert_eq!(field.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(field.cost, cost![1, W]);
        assert_eq!(field.colors(), vec![cards::Color::White]);
        assert_eq!(
            field.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: CREATURE,
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(rules::DeclareBlocker::ID),
                    change: rules::Change::OnlyIf {
                        role: rules::Role::Subject,
                        condition: rules::at_least(rules::Characteristic::Power, 3),
                    },
                }),
            }]
        );
    }
}
