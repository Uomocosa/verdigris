#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardType {
    Creature,
    Instant,
    Sorcery,
    Land,
    Artifact,
    Enchantment,
    Planeswalker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Indestructible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManaSymbol {
    White,
    Blue,
    Black,
    Red,
    Green,
    Generic(u32),
}
