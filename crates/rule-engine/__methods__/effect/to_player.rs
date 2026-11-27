use crate::effects;
use crate::rules;

pub fn to_player(effect: effects::Effect, player: impl Into<rules::Players>) -> effects::Effect {
    effects::Effect {
        affected: Some(rules::Subject::Each(player.into())),
        ..effect
    }
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::effects;
    use crate::rules;

    use super::to_player;

    #[test]
    fn test_usage() {
        let effect = to_player(effects::gain_life(1), YOU);
        assert_eq!(effect.kind, effects::EffectKind::GainLife(1));
        assert_eq!(
            effect.affected,
            Some(rules::Subject::Each(rules::Players::You))
        );
        assert_eq!(effect.target, None);
    }
}
