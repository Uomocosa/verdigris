use crate::effects;
use crate::keywords;
use crate::turns;

#[must_use]
pub const fn damage(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Damage(amount),
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn destroy() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Destroy,
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn exile() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Exile,
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn counter() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Counter,
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn creates(count: u32, definition: effects::TokenDefinition) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::CreateTokens { count, definition },
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn draw(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Draw(count),
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn mills(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Mill(count),
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn lose_life(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::LoseLife(amount),
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn poison(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Poison(amount),
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn gain_life(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::GainLife(amount),
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn discard(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Discard(count),
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn gets(power: i32, toughness: i32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::ModifyPowerToughness { power, toughness },
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn gains(keyword: keywords::StaticKeyword) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::GainKeyword(keyword),
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn additional_phase(phase: turns::Phase) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::AdditionalPhase(phase),
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn win_the_game() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::WinTheGame,
        follow_ups: [None; 4],
        target: None,
    }
}

#[must_use]
pub const fn target(count: u32, filter: effects::Filter) -> effects::Target {
    effects::Target { count, filter }
}
