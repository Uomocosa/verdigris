use crate::testing::prelude::*;

pub const TEST_KALITAS: Card = Card {
    name: TestCardName("Test Kalitas"),
    definition: || {
        card! {
            cards::creature("Test Kalitas", cost![2, B, B], pt!(3, 4));
            if_a(creature() & !token() & controlled_by(OPPONENT))
                .would::<PutIntoGraveyard>()
                .instead(PutIntoGraveyard::OBJECT.exile()),
        }
    },
};
