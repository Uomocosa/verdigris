use crate::PLAYER;
use crate::cards;
use crate::cost;
use crate::counters;
use crate::effects;
use crate::testing;

pub const CARESS_OF_PHYREXIA: testing::Card = testing::Card {
    name: testing::TestCardName("Caress of Phyrexia"),
    definition: || {
        cards::sorcery("Caress of Phyrexia", cost![3, B, B]).spell(
            effects::target(1, PLAYER).does(
                effects::draw(3) & effects::lose_life(3) & effects::gets(3 * counters::POISON),
            ),
        )
    },
};
