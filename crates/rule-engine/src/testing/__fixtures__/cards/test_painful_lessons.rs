use crate::PLAYER;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::rules;
use crate::rules::traits::event::Event;
use crate::statics;
use crate::testing;

pub const TEST_PAINFUL_LESSONS: testing::Card = testing::Card {
    name: testing::TestCardName("Test Painful Lessons"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Painful Lessons", cost![2, B]);
            statics::all(PLAYER).instead(
                events::Draw::ID,
                rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 2),
                vec![rules::EventTemplate::LoseLife {
                    who: rules::Role::Player,
                    amount: rules::Amount::FromEvent(rules::Field::Amount),
                }],
            ),
        }
    },
};
