use crate::testing::prelude::*;

pub const TEST_OPENING_TOLL: Card = Card {
    name: TestCardName("Test Opening Toll"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Opening Toll", cost![2]);
            opening_hand_action(vec![EventTemplate::Discard {
                who: Hooked::<OpeningHandAction>::PLAYER.into(),
                amount: Amount::Fixed(1),
            }]),
        }
    },
};
