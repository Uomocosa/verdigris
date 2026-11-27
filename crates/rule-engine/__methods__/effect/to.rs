use crate::effects;

pub fn to(effect: effects::Effect, target: impl Into<effects::Target>) -> effects::Effect {
    effects::Effect {
        target: Some(target.into()),
        ..effect
    }
}

#[cfg(test)]
mod tests {
    use crate::ANY_TARGET;
    use crate::THIS;
    use crate::effects;

    use super::to;

    #[test]
    fn test_usage() {
        let effect = to(effects::damage(3), effects::target(1, ANY_TARGET));
        assert_eq!(effect.kind, effects::EffectKind::Damage(3));
        assert_eq!(effect.target, Some(effects::target(1, ANY_TARGET)));
    }

    #[test]
    fn to_this_targets_the_sources_own_creature() {
        let effect = to(effects::gets(3, 3), THIS);
        let target = effect.target.unwrap();
        assert_eq!(target.count, 1);
        assert_eq!(target.filter.kind, effects::FilterKind::Creature);
        assert_eq!(
            target.filter.object,
            Some(effects::ObjectRelation::ThisObject)
        );
    }
}
