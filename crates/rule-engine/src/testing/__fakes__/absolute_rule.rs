use atomic_delegate_macros::atomic_delegate;

use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub struct AbsoluteRule;

impl LegalityRule for AbsoluteRule {
    type Ctx = rules::AttackCtx;
    const ID: rules::RuleId = rules::RuleId("test/absolute");

    #[atomic_delegate(AbsoluteRule, LegalityRule)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;
    use crate::testing;

    use super::AbsoluteRule;

    #[test]
    fn test_usage() {
        assert_eq!(AbsoluteRule::ID, rules::RuleId("test/absolute"));
        let clauses = AbsoluteRule::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, testing::UNTAPPED.key);
        assert!(clauses[0].absolute);
    }
}
