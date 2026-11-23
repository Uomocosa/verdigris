use crate::cards;
use crate::rules;
use crate::testing;

pub fn event_clauses() -> Vec<rules::Clause> {
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

    use super::event_clauses;

    #[test]
    fn test_usage() {
        let clauses = event_clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, rules::ClauseKey("test/untapped"));
        assert_eq!(clauses[0].condition, !rules::has::<cards::Tapped>());
    }
}
