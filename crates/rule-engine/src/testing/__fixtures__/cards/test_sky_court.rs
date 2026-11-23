use crate::CREATURE;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_SKY_COURT: testing::Card = testing::Card {
    name: testing::TestCardName("Test Sky Court"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Sky Court", cost![2, U]);
            statics::all(CREATURE).restriction::<rules::DeclareBlocker>(
                !rules::has::<cards::Flying>() | rules::other(rules::has::<cards::Flying>()),
            ),
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
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::TEST_SKY_COURT;

    #[test]
    fn test_usage() {
        let court = (TEST_SKY_COURT.definition)();
        assert_eq!(*court.name, "Test Sky Court");
        assert_eq!(court.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(court.cost, cost![2, U]);
        assert_eq!(court.colors(), vec![cards::Color::Blue]);
        assert_eq!(
            court.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: CREATURE,
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(rules::DeclareBlocker::ID),
                    change: rules::Change::OnlyIf {
                        role: rules::Role::Subject,
                        condition: !rules::has::<cards::Flying>()
                            | rules::other(rules::has::<cards::Flying>()),
                    },
                }),
            }]
        );
    }
}
