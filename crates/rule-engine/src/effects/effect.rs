use atomic_delegate_macros::atomic_delegates;

use crate::effects;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    pub kind: effects::EffectKind,
    pub follow_ups: [Option<effects::EffectKind>; 4],
    pub target: Option<effects::Target>,
    pub affected: Option<rules::Subject>,
}

impl std::ops::BitAnd for Effect {
    type Output = Self;

    fn bitand(self, next: Self) -> Self {
        let Self {
            kind,
            follow_ups,
            target,
            affected,
        } = self;
        match follow_ups {
            [None, second, third, fourth] => Self {
                kind,
                follow_ups: [Some(next.kind), second, third, fourth],
                target,
                affected,
            },
            [first, None, third, fourth] => Self {
                kind,
                follow_ups: [first, Some(next.kind), third, fourth],
                target,
                affected,
            },
            [first, second, None, fourth] => Self {
                kind,
                follow_ups: [first, second, Some(next.kind), fourth],
                target,
                affected,
            },
            [first, second, third, None] => Self {
                kind,
                follow_ups: [first, second, third, Some(next.kind)],
                target,
                affected,
            },
            _ => {
                Self::assert_chain_has_room(&follow_ups);
                Self {
                    kind,
                    follow_ups,
                    target,
                    affected,
                }
            }
        }
    }
}

#[atomic_delegates]
impl Effect {
    #[must_use]
    pub fn to(self, target: impl Into<effects::Target>) -> Self {}
}

impl Effect {
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
    use crate::THIS;
    use crate::counters;
    use crate::effects;
    use crate::rules;

    #[test]
    fn test_usage() {
        let effect = THIS.deals(rules::damage(3)).to(effects::target(1, ANY_TARGET));
        assert_eq!(effect.kind, effects::EffectKind::Damage(3));
        assert_eq!(effect.target, Some(effects::target(1, ANY_TARGET)));
        let sequence = effects::draw(2) & effects::lose_life(2);
        assert_eq!(sequence.kind, effects::EffectKind::Draw(2));
        assert_eq!(
            sequence.follow_ups[0],
            Some(effects::EffectKind::LoseLife(2))
        );
        let longer = sequence & effects::get(3 * counters::POISON);
        assert_eq!(
            longer.follow_ups[1],
            Some(effects::EffectKind::Poison(3 * counters::POISON))
        );
        let full = longer & effects::gain_life(1) & effects::discard(1);
        assert_eq!(full.follow_ups[2], Some(effects::EffectKind::GainLife(1)));
        assert_eq!(full.follow_ups[3], Some(effects::EffectKind::Discard(1)));
    }

    #[test]
    #[should_panic(expected = "an Effect chain holds at most 4 follow-up effects")]
    fn a_fifth_follow_up_overflows_the_chain() {
        let _ = effects::draw(1)
            & effects::lose_life(1)
            & effects::gain_life(1)
            & effects::mill(1)
            & effects::discard(1)
            & effects::get(1 * counters::POISON);
    }
}
