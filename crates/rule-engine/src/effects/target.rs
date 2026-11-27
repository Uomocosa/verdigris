use crate::creature;
use crate::effects;
use crate::effects::traits::gets::Gets;
use crate::keywords;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    pub count: u32,
    pub filter: effects::Filter,
}

impl From<rules::This> for Target {
    fn from(_: rules::This) -> Self {
        Self {
            count: 1,
            filter: creature() & effects::Filter::source(),
        }
    }
}

#[rustfmt::skip]
impl Target {
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
    #[must_use]
    pub fn gain(self, kw: keywords::Keyword) -> effects::Effect { self.do_(effects::gain(kw)) }
}

impl Target {
    #[must_use]
    pub fn do_(self, effect: effects::Effect) -> effects::Effect {
        if effect.target.is_some() {
            effect
        } else {
            effects::Effect {
                target: Some(self),
                ..effect
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::creature;
    use crate::effects;
    use crate::keywords;
    use crate::pt;

    use super::Target;

    #[test]
    fn test_usage() {
        let target = effects::target(1, PLAYER);
        let effect = target.clone().discard(2);
        assert_eq!(effect.kind, effects::EffectKind::Discard(2));
        assert_eq!(effect.target, Some(target));
        assert_eq!(effect.affected, None);
        let drawn = effects::target(1, PLAYER).draw(3);
        assert_eq!(drawn.kind, effects::EffectKind::Draw(3));
        assert_eq!(drawn.target, Some(effects::target(1, PLAYER)));
        let gained = effects::target(1, PLAYER).gain_life(1);
        assert_eq!(gained.kind, effects::EffectKind::GainLife(1));
        let lost = effects::target(1, PLAYER).lose_life(3);
        assert_eq!(lost.kind, effects::EffectKind::LoseLife(3));
        let milled = effects::target(1, PLAYER).mill(5);
        assert_eq!(milled.kind, effects::EffectKind::Mill(5));
        let won = effects::target(1, PLAYER).win_the_game();
        assert_eq!(won.kind, effects::EffectKind::WinTheGame);
        let grown = effects::target(1, creature()).get(pt!(3, 3));
        assert_eq!(
            grown.kind,
            effects::EffectKind::ModifyPowerToughness {
                power: 3,
                toughness: 3
            }
        );
        let granted = effects::target(1, creature()).gain(keywords::hexproof());
        assert_eq!(
            granted.target,
            Some(Target {
                count: 1,
                filter: creature(),
            })
        );
        let chained = effects::draw(3) & effects::lose_life(3);
        let done = effects::target(1, PLAYER).do_(chained);
        assert_eq!(done.target, Some(effects::target(1, PLAYER)));
        assert_eq!(done.follow_ups[0], Some(effects::EffectKind::LoseLife(3)));
    }
}
