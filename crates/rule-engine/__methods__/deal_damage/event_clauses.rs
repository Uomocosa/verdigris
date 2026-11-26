use crate::events;
use crate::rules;

pub fn event_clauses() -> Vec<rules::Clause> {
    vec![events::deal_damage::NONZERO_DAMAGE.clause(
        "If a source would deal 0 damage, it does not deal damage at all.",
        rules::Role::Subject,
        rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 1),
    )]
}

#[cfg(test)]
mod tests {
    use crate::rules;

    use super::event_clauses;

    #[test]
    fn test_usage() {
        let clauses = event_clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, rules::ClauseKey("120.8/nonzero"));
        assert_eq!(
            clauses[0].condition,
            rules::event_field(rules::Field::Amount, rules::Comparison::AtLeast, 1)
        );
    }
}
