use crate::rules::traits::event::Event;
use crate::testing::prelude::*;

pub const TEST_PAINFUL_LESSONS: Card = Card {
    name: TestCardName("Test Painful Lessons"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Painful Lessons", cost![2, B]);
            all(PLAYER).instead(
                Draw::ID,
                Draw::PLAYER,
                Draw::AMOUNT.is(Value(2).or_more()),
                vec![EventTemplate::LoseLife {
                    who: Draw::PLAYER.into(),
                    amount: Amount::FromArg(Draw::AMOUNT),
                }],
            ),
        }
    },
};
