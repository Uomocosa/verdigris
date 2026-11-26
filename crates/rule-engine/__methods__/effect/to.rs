use crate::effects;

pub fn to(effect: effects::Effect, target: effects::Target) -> effects::Effect {
    effects::Effect {
        target: Some(target),
        ..effect
    }
}

#[cfg(test)]
mod tests {
    use crate::ANY_TARGET;
    use crate::effects;

    use super::to;

    #[test]
    fn test_usage() {
        let effect = to(effects::damage(3), effects::target(1, ANY_TARGET));
        assert_eq!(effect.kind, effects::EffectKind::Damage(3));
        assert_eq!(effect.target, Some(effects::target(1, ANY_TARGET)));
    }
}
