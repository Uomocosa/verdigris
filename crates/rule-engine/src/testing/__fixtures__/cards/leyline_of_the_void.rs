use crate::abilities;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;
use crate::testing;

pub const LEYLINE_OF_THE_VOID: testing::Card = testing::Card {
    name: testing::TestCardName("Leyline of the Void"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Leyline of the Void", cost![2, B, B]);
            abilities::begin_the_game_with_it_on_the_battlefield(),
            statics::each_opponent().instead(
                events::PutIntoGraveyard::ID,
                rules::Condition::True,
                vec![rules::EventTemplate::Exile { what: rules::Role::Subject }],
            ),
        }
    },
};
