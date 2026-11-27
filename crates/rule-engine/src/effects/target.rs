use crate::effects;
use crate::effects::traits::gets::Gets;
use crate::keywords;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Target {
    pub count: u32,
    pub filter: effects::Filter,
}

impl From<rules::This> for Target {
    fn from(_: rules::This) -> Self {
        Self {
            count: 1,
            filter: effects::Filter {
                kind: effects::FilterKind::Creature,
                controlled_by: None,
                excluded_color: None,
                attacking: false,
                nontoken: false,
                object: Some(effects::ObjectRelation::ThisObject),
            },
        }
    }
}

#[rustfmt::skip]
impl Target {
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
    #[must_use]
    pub fn gains(self, kw: keywords::Keyword) -> effects::Effect { self.does(effects::gains(kw)) }
}

impl Target {
    #[must_use]
    pub fn does(self, effect: effects::Effect) -> effects::Effect {
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
    use crate::CREATURE;
    use crate::PLAYER;
    use crate::effects;
    use crate::keywords;
    use crate::pt;

    use super::Target;

    #[test]
    fn test_usage() {
        let target = effects::target(1, PLAYER);
        let effect = target.discards(2);
        assert_eq!(effect.kind, effects::EffectKind::Discard(2));
        assert_eq!(effect.target, Some(target));
        assert_eq!(effect.affected, None);
        let drawn = effects::target(1, PLAYER).draws(3);
        assert_eq!(drawn.kind, effects::EffectKind::Draw(3));
        assert_eq!(drawn.target, Some(effects::target(1, PLAYER)));
        let gained = effects::target(1, PLAYER).gains_life(1);
        assert_eq!(gained.kind, effects::EffectKind::GainLife(1));
        let lost = effects::target(1, PLAYER).loses_life(3);
        assert_eq!(lost.kind, effects::EffectKind::LoseLife(3));
        let milled = effects::target(1, PLAYER).mills(5);
        assert_eq!(milled.kind, effects::EffectKind::Mill(5));
        let won = effects::target(1, PLAYER).wins_the_game();
        assert_eq!(won.kind, effects::EffectKind::WinTheGame);
        let grown = effects::target(1, CREATURE).gets(pt!(3, 3));
        assert_eq!(
            grown.kind,
            effects::EffectKind::ModifyPowerToughness {
                power: 3,
                toughness: 3
            }
        );
        let granted = effects::target(1, CREATURE).gains(keywords::hexproof());
        assert_eq!(
            granted.target,
            Some(Target {
                count: 1,
                filter: CREATURE,
            })
        );
        let chained = effects::draw(3) & effects::lose_life(3);
        let done = effects::target(1, PLAYER).does(chained);
        assert_eq!(done.target, Some(effects::target(1, PLAYER)));
        assert_eq!(done.follow_ups[0], Some(effects::EffectKind::LoseLife(3)));
    }
}
