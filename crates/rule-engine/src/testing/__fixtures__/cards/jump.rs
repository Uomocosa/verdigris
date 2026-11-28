use crate::testing::prelude::*;

pub const JUMP: Card = Card {
    name: TestCardName("Jump"),
    definition: || cards::instant("Jump", cost![U]).spell(target(1, creature()).gain(flying())),
};
