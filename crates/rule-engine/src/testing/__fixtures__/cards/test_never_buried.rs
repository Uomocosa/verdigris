use crate::testing::prelude::*;

pub const TEST_NEVER_BURIED: Card = Card {
    name: TestCardName("Test Never Buried"),
    definition: || {
        card! {
            cards::creature("Test Never Buried", cost![1, G], pt!(2, 2));
            Replacement::Rule(Box::new(RuleChange {
                target: Target::event::<PutIntoGraveyard>(),
                change: Change::Instead {
                    holder: PutIntoGraveyard::OBJECT.into(),
                    when: always(),
                    then: Then::Events(Vec::new()),
                },
            })),
        }
    },
};
