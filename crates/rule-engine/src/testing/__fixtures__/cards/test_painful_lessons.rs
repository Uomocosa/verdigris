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
                events::Draw::PLAYER,
                events::Draw::AMOUNT.is(rules::Value(2).or_more()),
                vec![rules::EventTemplate::LoseLife {
                    who: events::Draw::PLAYER.into(),
                    amount: rules::Amount::FromArg(events::Draw::AMOUNT),
                }],
            ),
        }
    },
};
