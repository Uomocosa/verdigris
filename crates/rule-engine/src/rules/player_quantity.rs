use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerQuantity {
    Life,
    PoisonCounters,
    CardsInHand,
    CardsInLibrary,
}

#[rustfmt::skip]
impl PlayerQuantity {
    #[must_use]
    pub const fn at_least(self, threshold: i32) -> rules::Condition {
        rules::Condition::PlayerValue(self, rules::Comparison::OrMore(threshold))
    }

    #[must_use]
    pub const fn at_most(self, threshold: i32) -> rules::Condition {
        rules::Condition::PlayerValue(self, rules::Comparison::OrLess(threshold))
    }

    #[must_use]
    pub const fn equal_to(self, threshold: i32) -> rules::Condition {
        rules::Condition::PlayerValue(self, rules::Comparison::Exactly(threshold))
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::PlayerQuantity;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::life().at_least(20),
            rules::Condition::PlayerValue(PlayerQuantity::Life, rules::Comparison::OrMore(20))
        );
        assert_eq!(
            rules::poison_counters().at_most(0),
            rules::Condition::PlayerValue(
                PlayerQuantity::PoisonCounters,
                rules::Comparison::OrLess(0),
            )
        );
        assert_eq!(
            PlayerQuantity::CardsInHand.equal_to(0),
            rules::Condition::PlayerValue(
                PlayerQuantity::CardsInHand,
                rules::Comparison::Exactly(0),
            )
        );
    }
}
