use crate::testing::prelude::*;

pub const SIGN_IN_BLOOD: Card = Card {
    name: TestCardName("Sign in Blood"),
    definition: || {
        cards::sorcery("Sign in Blood", cost![B, B])
            .spell(target(1, PLAYER).do_(draw(2) & lose_life(2)))
    },
};
