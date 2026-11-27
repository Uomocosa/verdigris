use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Characteristic {
    Power,
    Toughness,
}

#[rustfmt::skip]
impl Characteristic {
    #[must_use]
    pub const fn is(self, comparison: rules::Comparison) -> rules::Condition {
        rules::Condition::Characteristic(self, comparison)
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::Characteristic;

    #[test]
    fn test_usage() {
        assert_eq!(
            Characteristic::Power.is(rules::Value(3).or_more()),
            rules::Condition::Characteristic(Characteristic::Power, rules::Comparison::OrMore(3),)
        );
    }
}
