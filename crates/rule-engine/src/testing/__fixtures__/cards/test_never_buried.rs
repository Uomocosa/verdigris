use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::pt;
use crate::rules;
use crate::testing;

pub const TEST_NEVER_BURIED: testing::Card = testing::Card {
    name: testing::TestCardName("Test Never Buried"),
    definition: || {
        card! {
            cards::creature("Test Never Buried", cost![1, G], pt!(2, 2));
            abilities::Replacement::Rule(rules::RuleChange {
                target: rules::Target::event::<events::PutIntoGraveyard>(),
                change: rules::Change::Instead {
                    when: rules::Condition::True,
                    then: rules::Then::Events(Vec::new()),
                },
            }),
        }
    },
};
