use crate::OPPONENT;
use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::statics;
use crate::testing;

pub const TEST_CALL_TO_ARMS: testing::Card = testing::Card {
    name: testing::TestCardName("Test Call to Arms"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Call to Arms", cost![1, R]);
            statics::all(creature() & effects::Filter::controlled_by(OPPONENT)).must_attack(),
        }
    },
};
