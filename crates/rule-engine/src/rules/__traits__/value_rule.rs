use crate::rules;

pub trait ValueRule: 'static {
    const ID: rules::RuleId;
    fn default() -> rules::Limit;
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::value_rule::ValueRule;

    #[test]
    fn test_usage() {
        assert_eq!(rules::MaximumHandSize::ID, rules::RuleId("402.2"));
        assert_eq!(rules::MaximumHandSize::default(), rules::Limit::At(7));
        assert_eq!(rules::LandPlays::ID, rules::RuleId("305.2"));
        assert_eq!(rules::LandPlays::default(), rules::Limit::At(1));
    }
}
