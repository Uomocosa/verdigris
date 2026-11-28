use crate::rules::traits::event::Event;
use crate::testing::prelude::*;

pub const LEYLINE_OF_THE_VOID: Card = Card {
    name: TestCardName("Leyline of the Void"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Leyline of the Void", cost![2, B, B]);
            begin_the_game_with_it_on_the_battlefield(),
            each_opponent().instead(
                PutIntoGraveyard::ID,
                PutIntoGraveyard::OBJECT.controller(),
                Condition::True,
                vec![EventTemplate::Exile { what: PutIntoGraveyard::OBJECT.into() }],
            ),
        }
    },
};
