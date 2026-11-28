use crate::testing::prelude::*;

pub const TEST_SILENT_ARBITER: Card = Card {
    name: TestCardName("Test Silent Arbiter"),
    definition: || {
        card! {
            cards::artifact_creature("Test Silent Arbiter", cost![4], pt!(1, 5));
            all(PLAYER).restriction::<DeclareAttackers>(
                DeclareAttackers::ATTACKERS.controller(),
                RestrictionId("test/silent-arbiter"),
                count(DeclareAttackers::ATTACKERS).is(Value(1).or_less()),
            ),
        }
    },
};
