use atomic_delegate_macros::atomic_delegates;

use crate::effects;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Effect {
    pub kind: effects::EffectKind,
    pub then: Option<effects::EffectKind>,
    pub target: Option<effects::Target>,
}

#[atomic_delegates]
impl Effect {
    #[must_use]
    pub fn to(self, target: effects::Target) -> Self {}
}

impl Effect {
    #[must_use]
    pub const fn and(mut self, next: Self) -> Self {
        self.then = Some(next.kind);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::ANY_TARGET;
    use crate::effects;

    #[test]
    fn test_usage() {
        let effect = effects::damage(3).to(effects::target(1, ANY_TARGET));
        assert_eq!(effect.kind, effects::EffectKind::Damage(3));
        assert_eq!(effect.target, Some(effects::target(1, ANY_TARGET)));
        let sequence = effects::draw(2).and(effects::lose_life(2));
        assert_eq!(sequence.kind, effects::EffectKind::Draw(2));
        assert_eq!(sequence.then, Some(effects::EffectKind::LoseLife(2)));
    }
}
