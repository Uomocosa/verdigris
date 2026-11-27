use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CharacteristicOf {
    pub characteristic: rules::Characteristic,
    pub subject: rules::Subject,
}

#[rustfmt::skip]
impl CharacteristicOf {
    #[must_use]
    pub fn is(self, comparison: rules::Comparison) -> rules::Condition {
        rules::Condition::Characteristic(self.characteristic, comparison).of(self.subject)
    }
}

#[cfg(test)]
mod tests {
    use crate::THIS;
    use crate::rules;

    use super::CharacteristicOf;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::power().of(THIS).is(rules::Value(3).or_more()),
            rules::Condition::Characteristic(
                rules::Characteristic::Power,
                rules::Comparison::OrMore(3),
            )
            .of(THIS),
        );
        assert_eq!(
            CharacteristicOf {
                characteristic: rules::Characteristic::Toughness,
                subject: THIS.into(),
            }
            .is(rules::Value(2).or_less()),
            rules::Condition::Characteristic(
                rules::Characteristic::Toughness,
                rules::Comparison::OrLess(2),
            )
            .of(THIS),
        );
    }
}
