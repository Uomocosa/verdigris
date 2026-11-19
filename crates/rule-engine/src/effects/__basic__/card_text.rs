use crate::effects;

#[must_use]
pub const fn damage(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Damage(amount),
        target: None,
    }
}

#[must_use]
pub const fn destroy() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Destroy,
        target: None,
    }
}

#[must_use]
pub const fn counter() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Counter,
        target: None,
    }
}

#[must_use]
pub const fn draw(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Draw(count),
        target: None,
    }
}

#[must_use]
pub const fn gets(power: i32, toughness: i32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::ModifyPowerToughness { power, toughness },
        target: None,
    }
}

#[must_use]
pub const fn target(count: u32, filter: effects::Filter) -> effects::Target {
    effects::Target { count, filter }
}
