use crate::HAND;
use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::cast_spell::PRIORITY.restriction(
            "A player who has priority",
            rules::has_priority(rules::Role::Player),
        ),
        events::cast_spell::FROM_HAND
            .restriction("from their hand", rules::in_zone(HAND, rules::Role::Player)),
        events::cast_spell::OWNS.restriction(
            "their hand",
            rules::points_to::<cards::Owner>(rules::Role::Player),
        ),
        events::cast_spell::NOT_A_LAND
            .restriction(
                "It can’t be cast as a spell.",
                !rules::card_type(cards::CardType::Land),
            )
            .unwaivable(),
        events::cast_spell::TIMING.restriction(
            "during a main phase of their turn when the stack is empty",
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

    use super::event_restrictions;

    #[test]
    fn test_usage() {
        let restrictions = event_restrictions();
        assert_eq!(restrictions.len(), 5);
        assert_eq!(restrictions[0].id, rules::RestrictionId("601.2a/priority"));
        assert_eq!(
            restrictions[0].condition,
            rules::has_priority(rules::Role::Player)
        );
        assert_eq!(restrictions[1].id, rules::RestrictionId("601.2a/from-hand"));
        assert_eq!(
            restrictions[1].condition,
            rules::in_zone(HAND, rules::Role::Player)
        );
        assert_eq!(restrictions[2].id, rules::RestrictionId("601.2a/owns"));
        assert_eq!(
            restrictions[2].condition,
            rules::points_to::<cards::Owner>(rules::Role::Player)
        );
        assert_eq!(
            restrictions[3].id,
            rules::RestrictionId("601.2a/not-a-land")
        );
        assert_eq!(
            restrictions[3].condition,
            !rules::card_type(cards::CardType::Land)
        );
        assert!(restrictions[3].unwaivable);
        assert_eq!(restrictions[4].id, rules::RestrictionId("601.2a/timing"));
        assert_eq!(
            restrictions[4].condition,
            rules::card_type(cards::CardType::Instant)
                | (rules::main_phase()
                    & rules::stack_empty()
                    & rules::active_player(rules::Role::Player))
        );
    }
}
