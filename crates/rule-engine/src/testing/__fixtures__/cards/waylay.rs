use crate::testing::prelude::*;

pub const KNIGHT_TOKEN: Card = Card {
    name: TestCardName("Knight"),
    definition: || {
        card! {
            cards::creature("Knight", cost![W], pt!(2, 2));
            at(
                TriggerEvent::beginning_of_next_cleanup(),
                exile(THIS),
            ),
        }
    },
};

pub const WAYLAY: Card = Card {
    name: TestCardName("Waylay"),
    definition: || {
        cards::instant("Waylay", cost![2, W]).spell(creates(3, KNIGHT_TOKEN.definition.into()))
    },
};
