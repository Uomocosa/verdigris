use crate::CREATURE;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::testing;

pub const TEST_FIRST_STRIKE_GRANT: testing::Card = testing::Card {
    name: testing::TestCardName("Test First Strike Grant"),
    definition: || {
        cards::instant("Test First Strike Grant", cost![R])
            .spell(effects::gains(keywords::first_strike()).to(effects::target(1, CREATURE)))
    },
};
