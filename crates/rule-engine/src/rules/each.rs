use derive_more::Deref;

use crate::effects;
use crate::effects::traits::gets::Gets;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct Each(pub rules::Players);

#[rustfmt::skip]
impl Each {
    #[must_use]
    pub fn draws(self, count: u32) -> effects::Effect { self.does(effects::draw(count)) }
    #[must_use]
    pub fn discards(self, count: u32) -> effects::Effect { self.does(effects::discard(count)) }
    #[must_use]
    pub fn gains_life(self, life: u32) -> effects::Effect { self.does(effects::gain_life(life)) }
    #[must_use]
    pub fn loses_life(self, life: u32) -> effects::Effect { self.does(effects::lose_life(life)) }
    #[must_use]
    pub fn mills(self, count: u32) -> effects::Effect { self.does(effects::mills(count)) }
    #[must_use]
    pub fn wins_the_game(self) -> effects::Effect { self.does(effects::win_the_game()) }
    #[must_use]
    pub fn gets(self, got: impl Gets) -> effects::Effect { self.does(effects::gets(got)) }
}

impl Each {
    #[must_use]
    pub fn does(self, effect: effects::Effect) -> effects::Effect {
        if effect.affected.is_some() {
            effect
        } else {
            effects::Effect {
                affected: Some(self.into()),
                ..effect
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::OPPONENT;
    use crate::effects;
    use crate::rules;

    use super::Each;

    #[test]
    fn test_usage() {
        let effect = rules::each(OPPONENT).draws(1);
        assert_eq!(effect.kind, effects::EffectKind::Draw(1));
        assert_eq!(
            effect.affected,
            Some(rules::Subject::Each(rules::Players::Opponents))
        );
        assert_eq!(effect.target, None);
        let each = Each(rules::Players::All);
        assert_eq!(each.discards(2).kind, effects::EffectKind::Discard(2));
        assert_eq!(each.gains_life(1).kind, effects::EffectKind::GainLife(1));
        assert_eq!(each.loses_life(2).kind, effects::EffectKind::LoseLife(2));
        assert_eq!(each.mills(5).kind, effects::EffectKind::Mill(5));
        assert_eq!(each.wins_the_game().kind, effects::EffectKind::WinTheGame);
        let chained = effects::draw(1) & effects::lose_life(1);
        let done = each.does(chained);
        assert_eq!(
            done.affected,
            Some(rules::Subject::Each(rules::Players::All))
        );
    }
}
