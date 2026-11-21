use crate::cards;
use crate::effects;

#[must_use]
pub const fn damage(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Damage(amount),
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn destroy() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Destroy,
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn counter() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Counter,
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn draw(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Draw(count),
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn lose_life(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::LoseLife(amount),
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn gain_life(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::GainLife(amount),
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn discard(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Discard(count),
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn gets(power: i32, toughness: i32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::ModifyPowerToughness { power, toughness },
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn gains(keyword: cards::Keyword) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::GainKeyword(keyword),
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn win_the_game() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::WinTheGame,
        then: None,
        target: None,
    }
}

#[must_use]
pub const fn target(count: u32, filter: effects::Filter) -> effects::Target {
    effects::Target { count, filter }
}
