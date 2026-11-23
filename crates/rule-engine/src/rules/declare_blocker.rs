use atomic_delegate_macros::atomic_delegate;

use crate::rules;
use crate::rules::traits::legality_rule::LegalityRule;

pub const UNTAPPED: rules::ClauseId<rules::BlockCtx> = rules::ClauseId::new("509.1a/untapped");
pub const FLYING: rules::ClauseId<rules::BlockCtx> = rules::ClauseId::new("702.9b/flying");

pub struct DeclareBlocker;

impl LegalityRule for DeclareBlocker {
    type Ctx = rules::BlockCtx;
    const ID: rules::RuleId = rules::RuleId("509.1a");

    #[atomic_delegate(DeclareBlocker, LegalityRule)]
    fn clauses() -> Vec<rules::Clause> {}
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::TAP_ATTACKERS;
    use crate::UNTAP_STEP;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::legality_rule::LegalityRule;

    use super::DeclareBlocker;
    use super::FLYING;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(DeclareBlocker::ID, rules::RuleId("509.1a"));
        let clauses = DeclareBlocker::clauses();
        assert_eq!(clauses.len(), 2);
        assert_eq!(clauses[0].key, UNTAPPED.key);
        assert_eq!(clauses[0].text, "The chosen creatures must be untapped");
        assert_eq!(clauses[0].role, rules::Role::Subject);
        assert_eq!(clauses[0].requires, !rules::has::<cards::Tapped>());
        assert_eq!(clauses[1].key, FLYING.key);
        assert_eq!(clauses[1].role, rules::Role::Subject);
        assert_eq!(
            clauses[1].requires,
            !rules::other(rules::has::<cards::Flying>()) | rules::has::<cards::Flying>()
        );
    }

    #[test]
    fn clause_keys_are_unique_across_declare_rules_and_step_ids_differ() {
        let mut seen = HashSet::new();
        for clause in DeclareBlocker::clauses().into_iter().chain(rules::DeclareAttacker::clauses())
        {
            assert!(
                seen.insert(clause.key),
                "duplicate clause key: {:?}",
                clause.key
            );
        }
        assert_ne!(TAP_ATTACKERS, UNTAP_STEP);
    }
}
