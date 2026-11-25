use crate::HAND;
use crate::events;
use crate::rules;

pub fn hook_event_clauses() -> Vec<rules::Clause> {
    vec![events::opening_hand_action::FROM_OPENING_HAND.clause(
        "Some cards allow a player to take actions with them from their opening hand.",
        rules::Role::Subject,
        rules::in_zone(HAND, rules::Role::Player),
    )]
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::rules;

    use super::hook_event_clauses;

    #[test]
    fn test_usage() {
        let clauses = hook_event_clauses();
        assert_eq!(clauses.len(), 1);
        assert_eq!(clauses[0].key, rules::ClauseKey("103.6/from-opening-hand"));
        assert_eq!(
            clauses[0].condition,
            rules::in_zone(HAND, rules::Role::Player)
        );
    }
}
