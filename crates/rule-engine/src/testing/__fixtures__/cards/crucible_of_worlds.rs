use crate::testing::prelude::*;

pub const CRUCIBLE_OF_WORLDS: Card = Card {
    name: TestCardName("Crucible of Worlds"),
    definition: || {
        card! {
            cards::CardDefinition::from_artifact("Crucible of Worlds", cost![3]);
            you().play_lands_from(GRAVEYARD),
        }
    },
};
