use crate::CREATURE;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_NO_BURIALS: testing::Card = testing::Card {
    name: testing::TestCardName("Test No Burials"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test No Burials", cost![2]);
            statics::all(CREATURE.controlled_by(YOU)).cant::<rules::PutIntoGraveyard>(rules::Role::Subject),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::YOU;
    use crate::abilities;
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::TEST_NO_BURIALS;

    #[test]
    fn test_usage() {
        let burials = (TEST_NO_BURIALS.definition)();
        assert_eq!(*burials.name, "Test No Burials");
        assert_eq!(burials.type_line.types, vec![cards::CardType::Enchantment]);
        assert_eq!(burials.cost, cost![2]);
        assert_eq!(
            burials.continuous_abilities().cloned().collect::<Vec<_>>(),
            [abilities::Continuous {
                filter: CREATURE.controlled_by(YOU),
                modification: effects::Modification::Rule(rules::RuleChange {
                    target: rules::Target::Event(rules::PutIntoGraveyard::ID),
                    change: rules::Change::Cant {
                        role: rules::Role::Subject
                    },
                }),
            },]
        );
    }
}
