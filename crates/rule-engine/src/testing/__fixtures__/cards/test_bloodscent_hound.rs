use crate::testing::prelude::*;

pub const TEST_BLOODSCENT_HOUND: Card = Card {
    name: TestCardName("Test Bloodscent Hound"),
    definition: || {
        card! {
            cards::creature("Test Bloodscent Hound", cost![1, R], pt!(2, 1));
            this().gain(haste()).as_long_as(life().of(an(OPPONENT)).is(Value(10).or_less())),
        }
    },
};
