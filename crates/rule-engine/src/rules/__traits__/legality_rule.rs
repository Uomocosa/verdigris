use crate::rules;
use crate::rules::traits::pair_ctx::PairCtx;

pub trait LegalityRule: 'static {
    type Ctx: PairCtx;
    const ID: rules::RuleId;
    fn clauses() -> Vec<rules::Clause>;
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    const UNTAPPED: rules::ClauseId<rules::BlockCtx> = rules::ClauseId::new("509.1a/untapped");

    #[test]
    fn test_usage() {
        struct TestRule;

        impl LegalityRule for TestRule {
            type Ctx = rules::BlockCtx;
            const ID: rules::RuleId = rules::RuleId("test/legality");

            fn clauses() -> Vec<rules::Clause> {
                vec![UNTAPPED.clause(
                    "The chosen creatures must be untapped",
                    rules::Role::Subject,
                    !rules::has::<cards::Tapped>(),
                )]
            }
        }

        assert_eq!(TestRule::ID, rules::RuleId("test/legality"));
        let clauses = TestRule::clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, UNTAPPED.key);
    }
}
