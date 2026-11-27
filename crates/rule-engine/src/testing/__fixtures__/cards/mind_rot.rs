use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const MIND_ROT: testing::Card = testing::Card {
    name: testing::TestCardName("Mind Rot"),
    definition: || {
        cards::sorcery("Mind Rot", cost![2, B]).spell(effects::target(1, PLAYER).discards(2))
    },
};
