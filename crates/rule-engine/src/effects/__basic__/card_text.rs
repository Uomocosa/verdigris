use crate::effects;
use crate::effects::traits::gets::Gets;
use crate::keywords;
use crate::turns;

#[must_use]
pub fn destroy(target: impl Into<effects::Target>) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Destroy,
        follow_ups: [None, None, None, None],
        target: Some(target.into()),
        affected: None,
    }
}

#[must_use]
pub fn exile(target: impl Into<effects::Target>) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Exile,
        follow_ups: [None, None, None, None],
        target: Some(target.into()),
        affected: None,
    }
}

#[must_use]
pub fn counter(target: impl Into<effects::Target>) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Counter,
        follow_ups: [None, None, None, None],
        target: Some(target.into()),
        affected: None,
    }
}

#[must_use]
pub const fn creates(count: u32, definition: effects::TokenDefinition) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::CreateTokens { count, definition },
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub const fn draw(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Draw(count),
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub const fn mill(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Mill(count),
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub const fn lose_life(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::LoseLife(amount),
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub fn get(got: impl Gets) -> effects::Effect {
    effects::Effect {
        kind: got.kind(),
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub const fn gain_life(amount: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::GainLife(amount),
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub const fn discard(count: u32) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::Discard(count),
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub const fn gain(keyword: keywords::Keyword) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::GainKeyword(keyword),
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub const fn additional_phase(phase: turns::Phase) -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::AdditionalPhase(phase),
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub const fn win_the_game() -> effects::Effect {
    effects::Effect {
        kind: effects::EffectKind::WinTheGame,
        follow_ups: [None, None, None, None],
        target: None,
        affected: None,
    }
}

#[must_use]
pub fn target(count: u32, filter: impl Into<effects::Filter>) -> effects::Target {
    effects::Target {
        count,
        filter: filter.into(),
    }
}
