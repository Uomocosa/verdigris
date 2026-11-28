use crate::testing::prelude::*;

pub const TEST_NO_BURIALS: Card = Card {
    name: TestCardName("Test No Burials"),
    definition: || {
        card! {
            cards::CardDefinition::from_enchantment("Test No Burials", cost![2]);
            all(creature() & controlled_by(YOU)).cant::<PutIntoGraveyard>(
                PutIntoGraveyard::OBJECT,
            ),
        }
    },
};
