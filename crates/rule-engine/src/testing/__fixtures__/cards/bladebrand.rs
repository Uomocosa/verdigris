use crate::CREATURE;
use crate::YOU;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::keywords;
use crate::testing;

pub const BLADEBRAND: testing::Card = testing::Card {
    name: testing::TestCardName("Bladebrand"),
    definition: || {
        cards::instant("Bladebrand", cost![1, B])
            .spell(effects::target(1, CREATURE).gain(keywords::deathtouch()) & YOU.draw(1))
    },
};
