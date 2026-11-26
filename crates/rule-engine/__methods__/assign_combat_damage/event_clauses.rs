use crate::cards;
use crate::events;
use crate::rules;

pub fn event_clauses() -> Vec<rules::Clause> {
    vec![events::assign_combat_damage::ONLY_BLOCKERS.clause(
        "A blocked creature assigns its combat damage to the creatures blocking it",
        rules::Role::Other,
        rules::points_to::<cards::Blocking>(rules::Role::Other),
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
        assert_eq!(clauses[0].key, rules::ClauseKey("510.1c/only-blockers"));
        assert_eq!(
            clauses[0].condition,
            rules::points_to::<cards::Blocking>(rules::Role::Other)
        );
    }
}
