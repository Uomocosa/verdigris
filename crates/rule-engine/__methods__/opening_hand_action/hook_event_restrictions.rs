use crate::HAND;
use crate::events;
use crate::rules;

pub fn hook_event_restrictions() -> Vec<rules::Restriction> {
    vec![events::opening_hand_action::FROM_OPENING_HAND.restriction(
        "Some cards allow a player to take actions with them from their opening hand.",
        rules::in_zone(HAND, rules::Role::Player),
    )]
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::rules;

    use super::hook_event_restrictions;

    #[test]
    fn test_usage() {
        let restrictions = hook_event_restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(
            restrictions[0].id,
            rules::RestrictionId("103.6/from-opening-hand")
        );
        assert_eq!(
            restrictions[0].condition,
            rules::in_zone(HAND, rules::Role::Player)
        );
    }
}
