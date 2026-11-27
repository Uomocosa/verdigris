use crate::THIS;
use crate::any_target;
use crate::cards;
use crate::effects;
use crate::rules;
use crate::testing;

pub const GUT_SHOT: testing::Card = testing::Card {
    name: testing::TestCardName("Gut Shot"),
    definition: || {
        cards::instant(
            "Gut Shot",
            cards::Cost::default().with_phyrexian(cards::Color::Red),
        )
        .spell(THIS.deals(rules::damage(1)).to(effects::target(1, any_target())))
    },
};
