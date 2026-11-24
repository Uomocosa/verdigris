use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::effects;
use crate::testing;

pub const CARESS_OF_PHYREXIA: testing::Card = testing::Card {
    name: testing::TestCardName("Caress of Phyrexia"),
    definition: || {
        cards::sorcery("Caress of Phyrexia", cost![3, B, B]).spell(
            effects::draw(3)
                .and(effects::lose_life(3))
                .and(effects::poison(3))
                .to(effects::target(1, PLAYER)),
        )
    },
};
