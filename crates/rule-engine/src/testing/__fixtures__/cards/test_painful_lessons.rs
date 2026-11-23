use crate::PLAYER;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;
use crate::testing;

pub const TEST_PAINFUL_LESSONS: testing::Card = testing::Card {
    name: testing::TestCardName("Test Painful Lessons"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Painful Lessons", cost![2, B]);
            statics::all(PLAYER).instead(
                rules::Draw::ID,
                rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 2),
                vec![rules::EventTemplate::LoseLife {
                    who: rules::Role::Player,
                    amount: rules::Amount::FromEvent(rules::Field::Amount),
                }],
            ),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::TEST_PAINFUL_LESSONS;

    #[test]
    fn test_usage() {
        let lessons = (TEST_PAINFUL_LESSONS.definition)();
        assert_eq!(*lessons.name, "Test Painful Lessons");
        assert_eq!(lessons.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(lessons.cost, cost![2, B]);
        assert_eq!(
            lessons.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: PLAYER,
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(rules::Draw::ID),
                    change: rules::Change::Instead {
                        when: rules::event_field(
                            rules::Field::Amount,
                            rules::Comparison::AtLeast,
                            2
                        ),
                        then: vec![rules::EventTemplate::LoseLife {
                            who: rules::Role::Player,
                            amount: rules::Amount::FromEvent(rules::Field::Amount),
                        }],
                    },
                }),
            }]
        );
    }
}
