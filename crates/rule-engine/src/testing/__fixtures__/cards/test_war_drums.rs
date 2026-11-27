use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::statics;
use crate::testing;

pub const TEST_WAR_DRUMS: testing::Card = testing::Card {
    name: testing::TestCardName("Test War Drums"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test War Drums", cost![2, R]);
            statics::all(creature() & effects::Filter::controlled_by(YOU)).must_attack(),
        }
    },
};
