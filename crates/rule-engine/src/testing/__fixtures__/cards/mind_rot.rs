use crate::testing::prelude::*;

pub const MIND_ROT: Card = Card {
    name: TestCardName("Mind Rot"),
    definition: || cards::sorcery("Mind Rot", cost![2, B]).spell(target(1, PLAYER).discard(2)),
};
