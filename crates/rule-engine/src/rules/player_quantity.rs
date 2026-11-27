use atomic_delegate_macros::atomic_delegates;

use crate::counters;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerQuantity {
    Life,
    Counters(counters::CounterId),
    CardsInHand,
    CardsInLibrary,
}

#[rustfmt::skip]
impl PlayerQuantity {
    #[must_use]
    pub const fn is(self, comparison: rules::Comparison) -> rules::Condition {
        rules::Condition::PlayerValue(self, comparison)
    }
}

#[atomic_delegates]
impl PlayerQuantity {
    #[must_use]
    pub fn of(self, player: impl Into<rules::Subject>) -> rules::PlayerQuantityOf {}
}

#[cfg(test)]
mod tests {
    use crate::counters;
    use crate::rules;

    use super::PlayerQuantity;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::life().is(rules::Value(20).or_more()),
            rules::Condition::PlayerValue(PlayerQuantity::Life, rules::Comparison::OrMore(20))
        );
        assert_eq!(
            rules::counters(counters::POISON).is(rules::Value(0).or_less()),
            rules::Condition::PlayerValue(
                PlayerQuantity::Counters(counters::POISON),
                rules::Comparison::OrLess(0),
            )
        );
        assert_eq!(
            PlayerQuantity::CardsInHand.is(rules::exactly(0)),
            rules::Condition::PlayerValue(
                PlayerQuantity::CardsInHand,
                rules::Comparison::Exactly(0),
            )
        );
    }
}
