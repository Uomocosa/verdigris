use crate::effects;
use crate::keywords;
use crate::rules;

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
    GainKeyword(keywords::StaticKeyword),
    WinTheGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterKind {
    Creature,
    Enchantment,
    Player,
    AnyTarget,
    Spell,
    CreatureSpell,
    NoncreatureSpell,
    InstantSpell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObjectRelation {
    ThisObject,
    EnchantedObject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layered {
    ModifyPT { power: i32, toughness: i32 },
    GainKeyword(keywords::StaticKeyword),
    LoseKeyword(keywords::StaticKeyword),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Change {
    Characteristic(Layered),
    Rule {
        question: rules::Question,
        answer: rules::Answer,
    },
    Waive(rules::ClauseKey),
    Restrict(rules::RuleId, rules::Predicate),
    Demand(rules::RuleId),
    Skip(rules::StepId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Duration {
    UntilEndOfTurn,
    WhileSourceOnBattlefield,
}
