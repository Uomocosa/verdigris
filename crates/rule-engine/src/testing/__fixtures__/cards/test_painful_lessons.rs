use crate::testing::prelude::*;

pub const TEST_PAINFUL_LESSONS: Card = Card {
    name: TestCardName("Test Painful Lessons"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Painful Lessons", cost![2, B]);
            if_a_player().would::<Draw>()
                .when(Draw::AMOUNT.is(Value(2).or_more()))
                .instead(Draw::PLAYER.lose_life(Draw::AMOUNT)),
        }
    },
};
