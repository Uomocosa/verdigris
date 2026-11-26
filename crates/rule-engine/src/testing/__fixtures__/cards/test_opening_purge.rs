use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::testing;

pub const TEST_OPENING_PURGE: testing::Card = testing::Card {
    name: testing::TestCardName("Test Opening Purge"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Opening Purge", cost![2]);
            abilities::opening_hand_action(vec![rules::EventTemplate::Discard {
                who: rules::Hooked::<events::OpeningHandAction>::PLAYER.into(),
                amount: rules::Amount::Fixed(7),
            }]),
        }
    },
};
