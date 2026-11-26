use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::testing;

pub const TEST_DOUBLE_GIFT: testing::Card = testing::Card {
    name: testing::TestCardName("Test Double Gift"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Double Gift", cost![2]);
            abilities::opening_hand_action(vec![rules::EventTemplate::GainLife {
                who: rules::Hooked::<events::OpeningHandAction>::PLAYER.into(),
                amount: rules::Amount::Fixed(1),
            }]),
            abilities::opening_hand_action(vec![rules::EventTemplate::GainLife {
                who: rules::Hooked::<events::OpeningHandAction>::PLAYER.into(),
                amount: rules::Amount::Fixed(2),
            }]),
        }
    },
};
