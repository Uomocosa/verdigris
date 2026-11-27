use crate::effects;
use crate::effects::traits::gets::Gets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct You;

#[rustfmt::skip]
impl You {
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

impl You {
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
    use crate::YOU;
    use crate::effects;
    use crate::pt;
    use crate::rules;

    use super::You;

    #[test]
    fn test_usage() {
        let effect = YOU.draw(2);
        assert_eq!(effect.kind, effects::EffectKind::Draw(2));
        assert_eq!(effect.affected, Some(rules::Subject::One(rules::YOU)));
        assert_eq!(effect.target, None);
        assert_eq!(You.discard(2).kind, effects::EffectKind::Discard(2));
        assert_eq!(You.gain_life(1).kind, effects::EffectKind::GainLife(1));
        assert_eq!(You.lose_life(2).kind, effects::EffectKind::LoseLife(2));
        assert_eq!(You.mill(5).kind, effects::EffectKind::Mill(5));
        assert_eq!(You.win_the_game().kind, effects::EffectKind::WinTheGame);
        assert_eq!(
            YOU.get(pt!(3, 3)).kind,
            effects::EffectKind::ModifyPowerToughness {
                power: 3,
                toughness: 3
            }
        );
        let chained = effects::draw(1) & effects::lose_life(1);
        let done = YOU.do_(chained);
        assert_eq!(done.affected, Some(rules::Subject::One(rules::YOU)));
        let kept = YOU.do_(YOU.draw(1));
        assert_eq!(kept.affected, Some(rules::Subject::One(rules::YOU)));
    }
}
