use crate::testing::prelude::*;

pub const TEST_OPENING_PURGE: Card = Card {
    name: TestCardName("Test Opening Purge"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Opening Purge", cost![2]);
            opening_hand_action(vec![EventTemplate::Discard {
                who: Hooked::<OpeningHandAction>::PLAYER.into(),
                amount: Amount::Fixed(7),
            }]),
        }
    },
};
