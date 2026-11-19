use crate::cards;

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub name: &'static str,
    pub definition: fn() -> cards::CardDefinition,
}
