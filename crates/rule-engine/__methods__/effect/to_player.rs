use crate::PLAYER;
use crate::effects;

pub fn to_player(effect: effects::Effect, player: effects::PlayerRef) -> effects::Effect {
    effects::Effect {
        target: Some(effects::target(1, PLAYER.controlled_by(player))),
        ..effect
    }
}

#[cfg(test)]
mod tests {
    use crate::YOU;
    use crate::effects;

    use super::to_player;

    #[test]
    fn test_usage() {
        let effect = to_player(effects::gain_life(1), YOU);
        assert_eq!(effect.kind, effects::EffectKind::GainLife(1));
        assert_eq!(
            effect.target,
            Some(effects::target(1, crate::PLAYER.controlled_by(YOU)))
        );
    }
}
