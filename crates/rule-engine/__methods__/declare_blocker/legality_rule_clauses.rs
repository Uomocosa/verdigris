use crate::cards;
use crate::rules;

pub fn legality_rule_clauses() -> Vec<rules::Clause> {
    vec![
        rules::UNTAPPED.clause(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        ),
        rules::FLYING.clause(
            "can’t be blocked except by creatures with flying and/or reach",
            rules::Role::Subject,
            !rules::other(rules::has::<cards::Flying>()) | rules::has::<cards::Flying>(),
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
        assert_eq!(clauses.len(), 2);
        assert_eq!(clauses[0].key, rules::ClauseKey("509.1a/untapped"));
        assert_eq!(clauses[0].requires, !rules::has::<cards::Tapped>());
        assert_eq!(clauses[1].key, rules::ClauseKey("702.9b/flying"));
        assert_eq!(
            clauses[1].requires,
            !rules::other(rules::has::<cards::Flying>()) | rules::has::<cards::Flying>()
        );
    }
}
