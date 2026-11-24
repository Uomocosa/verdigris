use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const TOME_SCOUR: testing::Card = testing::Card {
    name: testing::TestCardName("Tome Scour"),
    definition: || {
        cards::sorcery("Tome Scour", cost![U])
            .spell(effects::mills(5).to(effects::target(1, PLAYER)))
    },
};
