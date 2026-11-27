use crate::counters;
use crate::effects;
use crate::keywords;
use crate::rules;
use crate::turns;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Poison(counters::CounterAmount),
    GainLife(u32),
    Discard(u32),
    ModifyPowerToughness {
        power: i32,
        toughness: i32,
    },
    GainKeyword(keywords::Keyword),
    AdditionalPhase(turns::Phase),
    WinTheGame,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Layered {
    ModifyPT { power: i32, toughness: i32 },
    GainKeyword(keywords::Keyword),
    LoseKeyword(keywords::Keyword),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Modification {
    Characteristic(Layered),
    Rule(rules::RuleChange),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Duration {
    UntilEndOfTurn,
    WhileSourceOnBattlefield,
}
