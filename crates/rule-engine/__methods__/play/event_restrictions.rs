use crate::HAND;
use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::play::PRIORITY.restriction(rules::has_priority(events::Play::PLAYER)),
        events::play::LAND_CARD
            .restriction(rules::card_type(cards::CardType::Land).of(events::Play::CARD)),
        events::play::FROM_HAND
            .restriction(rules::in_zone(HAND, events::Play::PLAYER).of(events::Play::CARD)),
        events::play::MAIN_PHASE.restriction(rules::main_phase()),
        events::play::EMPTY_STACK.restriction(rules::stack_empty()),
        events::play::OWNS.restriction(
            rules::points_to::<cards::Owner>(events::Play::PLAYER).of(events::Play::CARD),
        ),
        events::play::OWN_TURN.restriction(rules::active_player(events::Play::PLAYER)).unwaivable(),
    ]
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::events;
    use crate::rules;

    use super::event_restrictions;

    #[test]
    fn test_usage() {
        let restrictions = event_restrictions();
        assert_eq!(restrictions.len(), 7);
        assert_eq!(restrictions[0].id, rules::RestrictionId("305.1/priority"));
        assert_eq!(
            restrictions[0].condition,
            rules::has_priority(events::Play::PLAYER)
        );
        assert_eq!(restrictions[1].id, rules::RestrictionId("305.1/land-card"));
        assert_eq!(
            restrictions[1].condition,
            rules::card_type(cards::CardType::Land).of(events::Play::CARD)
        );
        assert_eq!(restrictions[2].id, rules::RestrictionId("305.1/from-hand"));
        assert_eq!(
            restrictions[2].condition,
            rules::in_zone(HAND, events::Play::PLAYER).of(events::Play::CARD)
        );
        assert_eq!(restrictions[3].id, rules::RestrictionId("305.1/main-phase"));
        assert_eq!(restrictions[3].condition, rules::main_phase());
        assert_eq!(
            restrictions[4].id,
            rules::RestrictionId("305.1/empty-stack")
        );
        assert_eq!(restrictions[4].condition, rules::stack_empty());
        assert_eq!(restrictions[5].id, rules::RestrictionId("305.1/owns"));
        assert_eq!(
            restrictions[5].condition,
            rules::points_to::<cards::Owner>(events::Play::PLAYER).of(events::Play::CARD)
        );
        assert_eq!(restrictions[6].id, rules::RestrictionId("305.3/own-turn"));
        assert_eq!(
            restrictions[6].condition,
            rules::active_player(events::Play::PLAYER)
        );
        assert!(restrictions[6].unwaivable);
    }
}
