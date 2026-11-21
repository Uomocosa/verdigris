use crate::cards;
use crate::effects;
use crate::turns;

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
    Deathtouch,
    Lifelink,
    Trample,
    Flying,
    Reach,
    Vigilance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Supertype {
    Basic,
    Legendary,
    Ongoing,
    Snow,
    World,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerEvent {
    Enters(cards::ObjectFilter),
    BeginningOfStep(turns::Step, effects::PlayerRef),
}
