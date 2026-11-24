use crate::PLAYER;
use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::pt;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;
use crate::testing;

pub const LABORATORY_MANIAC: testing::Card = testing::Card {
    name: testing::TestCardName("Laboratory Maniac"),
    definition: || {
        card! {
            cards::creature("Laboratory Maniac", cost![2, U], pt!(2, 2))
                .subtype(cards::CreatureType::Human.into())
                .subtype(cards::CreatureType::Wizard.into());
            statics::all(PLAYER.controlled_by(YOU)).instead(
                rules::DrawFromEmptyLibrary::ID,
                rules::Predicate::True,
                vec![rules::EventTemplate::WinGame {
                    who: rules::Role::Player,
                }],
            ),
        }
    },
};
