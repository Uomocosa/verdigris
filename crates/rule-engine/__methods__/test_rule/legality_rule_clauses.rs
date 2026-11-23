use crate::cards;
use crate::rules;
use crate::testing;

pub fn legality_rule_clauses() -> Vec<rules::Clause> {
    vec![testing::UNTAPPED.clause(
        "must be untapped",
        rules::Role::Subject,
        !rules::has::<cards::Tapped>(),
    )]
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::legality_rule_clauses;

    #[test]
    fn test_usage() {
        let clauses = legality_rule_clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, rules::ClauseKey("test/untapped"));
        assert_eq!(clauses[0].condition, !rules::has::<cards::Tapped>());
    }
}
