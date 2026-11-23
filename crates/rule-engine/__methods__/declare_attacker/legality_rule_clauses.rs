use crate::cards;
use crate::rules;

pub fn legality_rule_clauses() -> Vec<rules::Clause> {
    vec![
        rules::declare_attacker::UNTAPPED.clause(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        ),
        rules::declare_attacker::SUMMONING_SICK.clause(
            "have been controlled by the active player continuously since the turn began",
            rules::Role::Subject,
            !rules::has::<cards::SummoningSick>(),
        ),
        rules::declare_attacker::DEFENDER.clause(
            "A creature with defender can’t attack.",
            rules::Role::Subject,
            !rules::has::<cards::Defender>(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::legality_rule_clauses;

    #[test]
    fn test_usage() {
        let clauses = legality_rule_clauses();
        assert_eq!(clauses.len(), 3);
        assert_eq!(clauses[0].key, rules::ClauseKey("508.1a/untapped"));
        assert_eq!(clauses[0].requires, !rules::has::<cards::Tapped>());
        assert_eq!(
            clauses[1].key,
            rules::ClauseKey("508.1a/controlled-since-turn-began")
        );
        assert_eq!(clauses[1].requires, !rules::has::<cards::SummoningSick>());
        assert_eq!(clauses[2].key, rules::ClauseKey("702.3b/defender"));
        assert_eq!(clauses[2].requires, !rules::has::<cards::Defender>());
    }
}
