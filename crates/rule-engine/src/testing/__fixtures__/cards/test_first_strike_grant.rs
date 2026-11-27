use crate::cards;
use crate::cost;
use crate::creature;
use crate::effects;
use crate::keywords;
use crate::testing;

pub const TEST_FIRST_STRIKE_GRANT: testing::Card = testing::Card {
    name: testing::TestCardName("Test First Strike Grant"),
    definition: || {
        cards::instant("Test First Strike Grant", cost![R])
            .spell(effects::target(1, creature()).gain(keywords::first_strike()))
    },
};
