use crate::testing::prelude::*;

pub const SCROLL_THIEF: Card = Card {
    name: TestCardName("Scroll Thief"),
    definition: || {
        card! {
            cards::creature("Scroll Thief", cost![2, U], pt!(1, 3));
            whenever(
                THIS.deals(COMBAT_DAMAGE).to(a(PLAYER)),
                draw(1),
            ),
        }
    },
};
