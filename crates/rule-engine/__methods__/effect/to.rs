use crate::effects;

pub fn to(effect: effects::Effect, target: impl Into<effects::Target>) -> effects::Effect {
    effects::Effect {
        target: Some(target.into()),
        ..effect
    }
}

#[cfg(test)]
mod tests {
    use crate::THIS;
    use crate::any_target;
    use crate::effects;
    use crate::pt;

    use super::to;

    #[test]
    fn test_usage() {
        let untargeted = effects::Effect {
            kind: effects::EffectKind::Damage(3),
            follow_ups: [None, None, None, None],
            target: None,
            affected: None,
        };
        let effect = to(untargeted, effects::target(1, any_target()));
        assert_eq!(effect.kind, effects::EffectKind::Damage(3));
        assert_eq!(effect.target, Some(effects::target(1, any_target())));
    }

    #[test]
    fn to_this_targets_the_sources_own_creature() {
        let effect = to(effects::get(pt!(3, 3)), THIS);
        let target = effect.target.unwrap();
        assert_eq!(target.count, 1);
        assert_eq!(target.filter, crate::creature() & effects::Filter::source());
    }
}
