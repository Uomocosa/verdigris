use crate::rules::traits::event::Event;
use crate::testing::prelude::*;

pub const LABORATORY_MANIAC: Card = Card {
    name: TestCardName("Laboratory Maniac"),
    definition: || {
        card! {
            cards::creature("Laboratory Maniac", cost![2, U], pt!(2, 2))
                .subtype(cards::HUMAN)
                .subtype(cards::WIZARD);
            you().instead(
                DrawFromEmptyLibrary::ID,
                DrawFromEmptyLibrary::PLAYER,
                always(),
                vec![EventTemplate::WinGame {
                    who: DrawFromEmptyLibrary::PLAYER.into(),
                }],
            ),
        }
    },
};
