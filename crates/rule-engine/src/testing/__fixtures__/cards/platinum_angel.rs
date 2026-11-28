use crate::testing::prelude::*;

pub const PLATINUM_ANGEL: Card = Card {
    name: TestCardName("Platinum Angel"),
    definition: || {
        card! {
            cards::artifact_creature("Platinum Angel", cost![7], pt!(4, 4))
                .subtype(cards::ANGEL);
            flying(),
            you().cant_lose_the_game(),
            each_opponent().cant_win_the_game(),
        }
    },
};
