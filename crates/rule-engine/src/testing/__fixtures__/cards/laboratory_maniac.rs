use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
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
            statics::you().instead(
                events::DrawFromEmptyLibrary::ID,
                rules::Condition::True,
                vec![rules::EventTemplate::WinGame {
                    who: rules::Role::Player,
                }],
            ),
        }
    },
};
