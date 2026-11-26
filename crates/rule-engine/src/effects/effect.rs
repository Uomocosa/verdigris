use atomic_delegate_macros::atomic_delegates;

use crate::effects;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    pub kind: effects::EffectKind,
    pub follow_ups: [Option<effects::EffectKind>; 4],
    pub target: Option<effects::Target>,
}

#[atomic_delegates]
impl Effect {
    #[must_use]
    pub fn to(self, target: effects::Target) -> Self {}

    #[must_use]
    pub fn to_player(self, player: effects::PlayerRef) -> Self {}
}

impl Effect {
    #[must_use]
    pub fn and(self, next: Self) -> Self {
        let Self {
            kind,
            follow_ups,
            target,
        } = self;
        match follow_ups {
            [None, second, third, fourth] => Self {
                kind,
                follow_ups: [Some(next.kind), second, third, fourth],
                target,
            },
            [first, None, third, fourth] => Self {
                kind,
                follow_ups: [first, Some(next.kind), third, fourth],
                target,
            },
            [first, second, None, fourth] => Self {
                kind,
                follow_ups: [first, second, Some(next.kind), fourth],
                target,
            },
            [first, second, third, None] => Self {
                kind,
                follow_ups: [first, second, third, Some(next.kind)],
                target,
            },
            _ => {
                Self::assert_chain_has_room(&follow_ups);
                Self {
                    kind,
                    follow_ups,
                    target,
                }
            }
        }
    }

    pub(crate) fn assert_chain_has_room(follow_ups: &[Option<effects::EffectKind>; 4]) {
        let is_full = matches!(follow_ups, [Some(_), Some(_), Some(_), Some(_)]);
        assert!(
            !is_full,
            "an Effect chain holds at most 4 follow-up effects"
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::ANY_TARGET;
    use crate::effects;

    #[test]
    fn test_usage() {
        let effect = effects::damage(3).to(effects::target(1, ANY_TARGET));
        assert_eq!(effect.kind, effects::EffectKind::Damage(3));
        assert_eq!(effect.target, Some(effects::target(1, ANY_TARGET)));
        let sequence = effects::draw(2).and(effects::lose_life(2));
        assert_eq!(sequence.kind, effects::EffectKind::Draw(2));
        assert_eq!(
            sequence.follow_ups[0],
            Some(effects::EffectKind::LoseLife(2))
        );
        let longer = sequence.and(effects::poison(3));
        assert_eq!(longer.follow_ups[1], Some(effects::EffectKind::Poison(3)));
        let full = longer.and(effects::gain_life(1)).and(effects::discard(1));
        assert_eq!(full.follow_ups[2], Some(effects::EffectKind::GainLife(1)));
        assert_eq!(full.follow_ups[3], Some(effects::EffectKind::Discard(1)));
    }

    #[test]
    #[should_panic(expected = "an Effect chain holds at most 4 follow-up effects")]
    fn a_fifth_follow_up_overflows_the_chain() {
        let _ = effects::draw(1)
            .and(effects::lose_life(1))
            .and(effects::gain_life(1))
            .and(effects::mills(1))
            .and(effects::discard(1))
            .and(effects::poison(1));
    }
}
