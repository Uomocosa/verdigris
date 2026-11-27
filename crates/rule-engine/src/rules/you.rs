use crate::effects;
use crate::effects::traits::gets::Gets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct You;

#[rustfmt::skip]
impl You {
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

impl You {
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
    use crate::YOU;
    use crate::effects;
    use crate::pt;
    use crate::rules;

    use super::You;

    #[test]
    fn test_usage() {
        let effect = YOU.draws(2);
        assert_eq!(effect.kind, effects::EffectKind::Draw(2));
        assert_eq!(effect.affected, Some(rules::Subject::One(rules::YOU)));
        assert_eq!(effect.target, None);
        assert_eq!(You.discards(2).kind, effects::EffectKind::Discard(2));
        assert_eq!(You.gains_life(1).kind, effects::EffectKind::GainLife(1));
        assert_eq!(You.loses_life(2).kind, effects::EffectKind::LoseLife(2));
        assert_eq!(You.mills(5).kind, effects::EffectKind::Mill(5));
        assert_eq!(You.wins_the_game().kind, effects::EffectKind::WinTheGame);
        assert_eq!(
            YOU.gets(pt!(3, 3)).kind,
            effects::EffectKind::ModifyPowerToughness {
                power: 3,
                toughness: 3
            }
        );
        let chained = effects::draw(1) & effects::lose_life(1);
        let done = YOU.does(chained);
        assert_eq!(done.affected, Some(rules::Subject::One(rules::YOU)));
        let kept = YOU.does(YOU.draws(1));
        assert_eq!(kept.affected, Some(rules::Subject::One(rules::YOU)));
    }
}
