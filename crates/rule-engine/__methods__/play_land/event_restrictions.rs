use crate::HAND;
use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::play_land::PRIORITY.restriction(
            "A player who has priority",
            rules::has_priority(events::PlayLand::PLAYER),
        ),
        events::play_land::LAND_CARD.restriction(
            "a land card",
            rules::card_type(cards::CardType::Land).of(events::PlayLand::LAND),
        ),
        events::play_land::FROM_HAND.restriction(
            "from their hand",
            rules::in_zone(HAND, events::PlayLand::PLAYER).of(events::PlayLand::LAND),
        ),
        events::play_land::MAIN_PHASE
            .restriction("during a main phase of their turn", rules::main_phase()),
        events::play_land::EMPTY_STACK.restriction("when the stack is empty", rules::stack_empty()),
        events::play_land::OWNS.restriction(
            "their hand",
            rules::points_to::<cards::Owner>(events::PlayLand::PLAYER).of(events::PlayLand::LAND),
        ),
        events::play_land::OWN_TURN
            .restriction(
                "A player can’t play a land, for any reason, if it isn’t their turn.",
                rules::active_player(events::PlayLand::PLAYER),
            )
            .unwaivable(),
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
            rules::has_priority(events::PlayLand::PLAYER)
        );
        assert_eq!(restrictions[1].id, rules::RestrictionId("305.1/land-card"));
        assert_eq!(
            restrictions[1].condition,
            rules::card_type(cards::CardType::Land).of(events::PlayLand::LAND)
        );
        assert_eq!(restrictions[2].id, rules::RestrictionId("305.1/from-hand"));
        assert_eq!(
            restrictions[2].condition,
            rules::in_zone(HAND, events::PlayLand::PLAYER).of(events::PlayLand::LAND)
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
            rules::points_to::<cards::Owner>(events::PlayLand::PLAYER).of(events::PlayLand::LAND)
        );
        assert_eq!(restrictions[6].id, rules::RestrictionId("305.3/own-turn"));
        assert_eq!(
            restrictions[6].condition,
            rules::active_player(events::PlayLand::PLAYER)
        );
        assert!(restrictions[6].unwaivable);
    }
}
