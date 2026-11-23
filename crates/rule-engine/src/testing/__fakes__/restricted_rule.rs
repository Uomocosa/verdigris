use atomic_delegate_macros::atomic_delegate;

use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub struct RestrictedRule;

impl LegalityRule for RestrictedRule {
    type Ctx = rules::AttackCtx;
    const ID: rules::EventId = rules::EventId("test/restriction");

    #[atomic_delegate(RestrictedRule, LegalityRule)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;
    use crate::testing;

    use super::RestrictedRule;

    #[test]
    fn test_usage() {
        assert_eq!(RestrictedRule::ID, rules::EventId("test/restriction"));
        let clauses = RestrictedRule::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, testing::UNTAPPED.key);
    }
}
