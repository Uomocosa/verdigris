use crate::testing::prelude::*;

pub const TOME_SCOUR: Card = Card {
    name: TestCardName("Tome Scour"),
    definition: || cards::sorcery("Tome Scour", cost![U]).spell(target(1, PLAYER).mill(5)),
};
