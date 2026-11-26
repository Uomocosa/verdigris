use crate::UNTAP;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_FROZEN_RELIC: testing::Card = testing::Card {
    name: testing::TestCardName("Test Frozen Relic"),
    definition: || {
        card! {
            cards::creature("Test Frozen Relic", cost![2], pt!(1, 1));
            statics::this().restriction::<events::Untap>(
                rules::RestrictionId("test/test-frozen-relic"),
                "untaps only during your untap step",
                !(rules::step_is(UNTAP) & rules::active_player(rules::Role::Player)),
            ),
        }
    },
};
