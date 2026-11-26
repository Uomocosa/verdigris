use crate::PLAYER;
use crate::card;
use crate::cards;
use crate::cost;
use crate::events;
use crate::pt;
use crate::rules;
use crate::statics;
use crate::testing;

pub const TEST_SILENT_ARBITER: testing::Card = testing::Card {
    name: testing::TestCardName("Test Silent Arbiter"),
    definition: || {
        card! {
            cards::artifact_creature("Test Silent Arbiter", cost![4], pt!(1, 5));
            statics::all(PLAYER).restriction::<events::DeclareAttackers>(
                events::DeclareAttackers::PLAYER,
                rules::RestrictionId("test/silent-arbiter"),
                rules::count(events::DeclareAttackers::ATTACKERS).at_most(1),
            ),
        }
    },
};
