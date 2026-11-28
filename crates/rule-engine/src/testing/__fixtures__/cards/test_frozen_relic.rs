use crate::testing::prelude::*;

pub const TEST_FROZEN_RELIC: Card = Card {
    name: TestCardName("Test Frozen Relic"),
    definition: || {
        card! {
            cards::creature("Test Frozen Relic", cost![2], pt!(1, 1));
            this().restriction::<Untap>(
                Untap::PERMANENT,
                RestrictionId("test/test-frozen-relic"),
                !(step_is(UNTAP) & active_player(Untap::PERMANENT.controller())),
            ),
        }
    },
};
