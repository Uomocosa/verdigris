use derive_more::Deref;

use crate::effects;
use crate::effects::traits::gets::Gets;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct Each(pub rules::Players);

#[rustfmt::skip]
impl Each {
    #[must_use]
    pub fn draw(self, count: u32) -> effects::Effect { self.do_(effects::draw(count)) }
    #[must_use]
    pub fn discard(self, count: u32) -> effects::Effect { self.do_(effects::discard(count)) }
    #[must_use]
    pub fn gain_life(self, life: u32) -> effects::Effect { self.do_(effects::gain_life(life)) }
    #[must_use]
    pub fn lose_life(self, life: u32) -> effects::Effect { self.do_(effects::lose_life(life)) }
    #[must_use]
    pub fn mill(self, count: u32) -> effects::Effect { self.do_(effects::mill(count)) }
    #[must_use]
    pub fn win_the_game(self) -> effects::Effect { self.do_(effects::win_the_game()) }
    #[must_use]
    pub fn get(self, got: impl Gets) -> effects::Effect { self.do_(effects::get(got)) }
}

impl Each {
    #[must_use]
    pub fn do_(self, effect: effects::Effect) -> effects::Effect {
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
        let effect = rules::each(OPPONENT).draw(1);
        assert_eq!(effect.kind, effects::EffectKind::Draw(1));
        assert_eq!(
            effect.affected,
            Some(rules::Subject::Each(rules::Players::Opponents))
        );
        assert_eq!(effect.target, None);
        let each = Each(rules::Players::All);
        assert_eq!(each.discard(2).kind, effects::EffectKind::Discard(2));
        assert_eq!(each.gain_life(1).kind, effects::EffectKind::GainLife(1));
        assert_eq!(each.lose_life(2).kind, effects::EffectKind::LoseLife(2));
        assert_eq!(each.mill(5).kind, effects::EffectKind::Mill(5));
        assert_eq!(each.win_the_game().kind, effects::EffectKind::WinTheGame);
        let chained = effects::draw(1) & effects::lose_life(1);
        let done = each.do_(chained);
        assert_eq!(
            done.affected,
            Some(rules::Subject::Each(rules::Players::All))
        );
    }
}
