use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const SIGN_IN_BLOOD: testing::Card = testing::Card {
    name: testing::TestCardName("Sign in Blood"),
    definition: || {
        cards::sorcery("Sign in Blood", cost![B, B])
            .spell(effects::draw(2).and(effects::lose_life(2)).to(effects::target(1, PLAYER)))
    },
};
