use crate::effects;
use crate::keywords;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectKind {
    Damage(u32),
    Destroy,
    Exile,
    Counter,
    CreateTokens {
        count: u32,
        definition: effects::TokenDefinition,
    },
    Draw(u32),
    Mill(u32),
    LoseLife(u32),
    Poison(u32),
    GainLife(u32),
    Discard(u32),
    ModifyPowerToughness {
        power: i32,
        toughness: i32,
    },
    GainKeyword(keywords::Keyword),
    WinTheGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterKind {
    Creature,
    Player,
    AnyTarget,
    Spell,
    CreatureSpell,
    NoncreatureSpell,
    InstantSpell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layered {
    ModifyPT { power: i32, toughness: i32 },
    GainKeyword(keywords::Keyword),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Duration {
    UntilEndOfTurn,
}
