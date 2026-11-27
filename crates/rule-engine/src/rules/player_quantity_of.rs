use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerQuantityOf {
    pub quantity: rules::PlayerQuantity,
    pub player: rules::ArgRef,
}

#[rustfmt::skip]
impl PlayerQuantityOf {
    #[must_use]
    pub fn is(self, comparison: rules::Comparison) -> rules::Condition {
        rules::Condition::PlayerValue(self.quantity, comparison).of(self.player)
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::life().of(rules::YOU).is(rules::Value(40).or_more()),
            rules::Condition::PlayerValue(
                rules::PlayerQuantity::Life,
                rules::Comparison::OrMore(40),
            )
            .of(rules::YOU),
        );
    }
}
