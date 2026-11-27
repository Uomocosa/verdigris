use crate::YOU;
use crate::card;
use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::statics;
use crate::testing;

pub const TEST_COVER_OF_DUSK: testing::Card = testing::Card {
    name: testing::TestCardName("Test Cover of Dusk"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test Cover of Dusk", cost![2, W]);
            statics::all(creature() & effects::Filter::controlled_by(YOU) & effects::Filter::attacking()).attack_without_tapping(),
        }
    },
};
