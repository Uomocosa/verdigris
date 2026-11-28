use crate::testing::prelude::*;

pub const GUT_SHOT: Card = Card {
    name: TestCardName("Gut Shot"),
    definition: || {
        cards::instant(
            "Gut Shot",
            cards::Cost::default().with_phyrexian(cards::Color::Red),
        )
        .spell(THIS.deals(damage(1)).to(target(1, any_target())))
    },
};
