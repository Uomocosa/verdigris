use crate::testing::prelude::*;

pub const TEST_LAST_STAND: Card = Card {
    name: TestCardName("Test Last Stand"),
    definition: || {
        card! {
            cards::creature("Test Last Stand", cost![2, B], pt!(3, 3));
            when(
                THIS.dies().if_(power().of(THIS).is(Value(3).or_more())),
                YOU.draw(1),
            ),
        }
    },
};
