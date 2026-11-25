use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::rules;
use crate::testing;

pub const TEST_MAY_PURGE: testing::Card = testing::Card {
    name: testing::TestCardName("Test May Purge"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test May Purge", cost![2]);
            abilities::opening_hand_action(rules::may(rules::EventTemplate::Discard {
                who: rules::Role::Player,
                amount: rules::Amount::Fixed(7),
            })),
        }
    },
};
