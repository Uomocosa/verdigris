use crate::ANY_TARGET;
use crate::cards;
use crate::effects;
use crate::testing;

pub const GUT_SHOT: testing::Card = testing::Card {
    name: testing::TestCardName("Gut Shot"),
    definition: || {
        cards::instant(
            "Gut Shot",
            cards::Cost::default().with_phyrexian(cards::Color::Red),
        )
        .spell(effects::damage(1).to(effects::target(1, ANY_TARGET)))
    },
};
