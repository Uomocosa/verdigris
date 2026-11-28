use crate::testing::prelude::*;

pub const DEMYSTIFY: Card = Card {
    name: TestCardName("Demystify"),
    definition: || cards::instant("Demystify", cost![W]).spell(destroy(target(1, enchantment()))),
};
