use atomic_delegate_macros::atomic_delegate;

use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub const UNTAPPED: rules::ClauseId<rules::AttackCtx> = rules::ClauseId::new("test/untapped");
pub const TEST_STEP: rules::StepId = rules::StepId("test/step");

pub struct TestRule;

impl LegalityRule for TestRule {
    type Ctx = rules::AttackCtx;
    const ID: rules::RuleId = rules::RuleId("test/legality");

    #[atomic_delegate(TestRule, LegalityRule)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::TEST_STEP;
    use super::TestRule;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(TestRule::ID, rules::RuleId("test/legality"));
        assert_eq!(*TEST_STEP, "test/step");
        let clauses = TestRule::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, UNTAPPED.key);
    }
}
