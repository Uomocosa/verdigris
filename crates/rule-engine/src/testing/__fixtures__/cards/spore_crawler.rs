use crate::testing::prelude::*;

pub const SPORE_CRAWLER: Card = Card {
    name: TestCardName("Spore Crawler"),
    definition: || {
        card! {
            cards::creature("Spore Crawler", cost![2, G], pt!(3, 2));
            when(THIS.dies(), draw(1)),
        }
    },
};
