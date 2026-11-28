use crate::testing::prelude::*;

pub const FELIDAR_SOVEREIGN: Card = Card {
    name: TestCardName("Felidar Sovereign"),
    definition: || {
        card! {
            cards::creature("Felidar Sovereign", cost![4, W, W], pt!(4, 6));
            vigilance(),
            lifelink(),
            at(
                beginning_of(UPKEEP).of(YOU).if_(life().of(YOU).is(Value(40).or_more())),
                win_the_game(),
            ),
        }
    },
};
