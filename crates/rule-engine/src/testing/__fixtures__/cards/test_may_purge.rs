use crate::testing::prelude::*;

pub const TEST_MAY_PURGE: Card = Card {
    name: TestCardName("Test May Purge"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test May Purge", cost![2]);
            opening_hand_action(may(EventTemplate::Discard {
                who: Hooked::<OpeningHandAction>::PLAYER.into(),
                amount: Amount::Fixed(7),
            })),
        }
    },
};
