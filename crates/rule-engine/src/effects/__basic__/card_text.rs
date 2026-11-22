use crate::effects;
use crate::keywords;

#[must_use]
pub const fn damage(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Damage(amount),
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn destroy() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Destroy,
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn counter() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Counter,
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn draw(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Draw(count),
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn lose_life(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::LoseLife(amount),
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn poison(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Poison(amount),
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn gain_life(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::GainLife(amount),
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn discard(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Discard(count),
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn gets(power: i32, toughness: i32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::ModifyPowerToughness { power, toughness },
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn gains(keyword: keywords::Keyword) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::GainKeyword(keyword),
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn win_the_game() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::WinTheGame,
        then: None,
        finally: None,
        target: None,
    }
}

#[must_use]
pub const fn target(count: u32, filter: effects::Filter) -> effects::Target {
    effects::Target { count, filter }
}
