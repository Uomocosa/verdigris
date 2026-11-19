use crate::cards;
use crate::testing;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub name: testing::TestCardName,
    pub definition: fn() -> cards::CardDefinition,
}
