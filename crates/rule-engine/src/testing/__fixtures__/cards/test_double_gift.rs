use crate::testing::prelude::*;

pub const TEST_DOUBLE_GIFT: Card = Card {
    name: TestCardName("Test Double Gift"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Double Gift", cost![2]);
            opening_hand_action(vec![EventTemplate::GainLife {
                who: Hooked::<OpeningHandAction>::PLAYER.into(),
                amount: Amount::Fixed(1),
            }]),
            opening_hand_action(vec![EventTemplate::GainLife {
                who: Hooked::<OpeningHandAction>::PLAYER.into(),
                amount: Amount::Fixed(2),
            }]),
        }
    },
};
