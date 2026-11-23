use crate::HAND;
use crate::cards;
use crate::rules;

pub fn event_clauses() -> Vec<rules::Clause> {
    vec![
        rules::cast_spell::PRIORITY.clause(
            "A player who has priority",
            rules::Role::Player,
            rules::has_priority(rules::Role::Player),
        ),
        rules::cast_spell::FROM_HAND.clause(
            "from their hand",
            rules::Role::Player,
            rules::in_zone(HAND, rules::Role::Player),
        ),
        rules::cast_spell::OWNS.clause(
            "their hand",
            rules::Role::Player,
            rules::points_to::<cards::Owner>(rules::Role::Player),
        ),
        rules::cast_spell::NOT_A_LAND
            .clause(
                "It can’t be cast as a spell.",
                rules::Role::Subject,
                !rules::card_type(cards::CardType::Land),
            )
            .absolute(),
        rules::cast_spell::TIMING.clause(
            "during a main phase of their turn when the stack is empty",
            rules::Role::Player,
            rules::card_type(cards::CardType::Instant)
                | (rules::main_phase()
                    & rules::stack_empty()
                    & rules::active_player(rules::Role::Player)),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::rules;

    use super::event_clauses;

    #[test]
    fn test_usage() {
        let clauses = event_clauses();
        assert_eq!(clauses.len(), 5);
        assert_eq!(clauses[0].key, rules::ClauseKey("601.2a/priority"));
        assert_eq!(
            clauses[0].condition,
            rules::has_priority(rules::Role::Player)
        );
        assert_eq!(clauses[1].key, rules::ClauseKey("601.2a/from-hand"));
        assert_eq!(
            clauses[1].condition,
            rules::in_zone(HAND, rules::Role::Player)
        );
        assert_eq!(clauses[2].key, rules::ClauseKey("601.2a/owns"));
        assert_eq!(
            clauses[2].condition,
            rules::points_to::<cards::Owner>(rules::Role::Player)
        );
        assert_eq!(clauses[3].key, rules::ClauseKey("601.2a/not-a-land"));
        assert_eq!(
            clauses[3].condition,
            !rules::card_type(cards::CardType::Land)
        );
        assert!(clauses[3].absolute);
        assert_eq!(clauses[4].key, rules::ClauseKey("601.2a/timing"));
        assert_eq!(
            clauses[4].condition,
            rules::card_type(cards::CardType::Instant)
                | (rules::main_phase()
                    & rules::stack_empty()
                    & rules::active_player(rules::Role::Player))
        );
    }
}
