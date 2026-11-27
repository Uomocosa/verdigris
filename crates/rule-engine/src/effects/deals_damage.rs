use derive_more::{Deref, From};

use crate::effects;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, From)]
pub struct DealsDamage(pub u32);

impl DealsDamage {
    #[must_use]
    pub fn to(self, target: impl Into<effects::Target>) -> effects::Effect {
        let kind = effects::EffectKind::Damage(*self);
        effects::Effect {
            kind,
            follow_ups: [None, None, None, None],
            target: Some(target.into()),
            affected: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::any_target;
    use crate::effects;

    use super::DealsDamage;

    #[test]
    fn test_usage() {
        let effect = DealsDamage::from(3u32).to(effects::target(1, any_target()));
        assert_eq!(effect.kind, effects::EffectKind::Damage(3));
        assert_eq!(effect.target, Some(effects::target(1, any_target())));
    }
}
