use crate::cards;
use crate::testing;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ZoneObject {
    Card {
        card: testing::TestCard,
        name: cards::CardName,
    },
    Trigger {
        ability: testing::TestTrigger,
        source_name: cards::CardName,
    },
}
