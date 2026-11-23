use crate::HAND;
use crate::cards;
use crate::rules;

pub fn event_clauses() -> Vec<rules::Clause> {
    vec![
        rules::play_land::PRIORITY.clause(
            "A player who has priority",
            rules::Role::Player,
            rules::has_priority(rules::Role::Player),
        ),
        rules::play_land::LAND_CARD.clause(
            "a land card",
            rules::Role::Subject,
            rules::card_type(cards::CardType::Land),
        ),
        rules::play_land::FROM_HAND.clause(
            "from their hand",
            rules::Role::Player,
            rules::in_zone(HAND, rules::Role::Player),
        ),
        rules::play_land::MAIN_PHASE.clause(
            "during a main phase of their turn",
            rules::Role::Player,
            rules::main_phase(),
        ),
        rules::play_land::EMPTY_STACK.clause(
            "when the stack is empty",
            rules::Role::Player,
            rules::stack_empty(),
        ),
        rules::play_land::OWNS.clause(
            "their hand",
            rules::Role::Subject,
            rules::points_to::<cards::Owner>(rules::Role::Player),
        ),
        rules::play_land::OWN_TURN
            .clause(
                "A player can’t play a land, for any reason, if it isn’t their turn.",
                rules::Role::Player,
                rules::active_player(rules::Role::Player),
            )
            .absolute(),
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
        assert_eq!(clauses.len(), 7);
        assert_eq!(clauses[0].key, rules::ClauseKey("305.1/priority"));
        assert_eq!(
            clauses[0].condition,
            rules::has_priority(rules::Role::Player)
        );
        assert_eq!(clauses[1].key, rules::ClauseKey("305.1/land-card"));
        assert_eq!(
            clauses[1].condition,
            rules::card_type(cards::CardType::Land)
        );
        assert_eq!(clauses[2].key, rules::ClauseKey("305.1/from-hand"));
        assert_eq!(
            clauses[2].condition,
            rules::in_zone(HAND, rules::Role::Player)
        );
        assert_eq!(clauses[3].key, rules::ClauseKey("305.1/main-phase"));
        assert_eq!(clauses[3].condition, rules::main_phase());
        assert_eq!(clauses[4].key, rules::ClauseKey("305.1/empty-stack"));
        assert_eq!(clauses[4].condition, rules::stack_empty());
        assert_eq!(clauses[5].key, rules::ClauseKey("305.1/owns"));
        assert_eq!(
            clauses[5].condition,
            rules::points_to::<cards::Owner>(rules::Role::Player)
        );
        assert_eq!(clauses[6].key, rules::ClauseKey("305.3/own-turn"));
        assert_eq!(
            clauses[6].condition,
            rules::active_player(rules::Role::Player)
        );
        assert!(clauses[6].absolute);
    }
}
