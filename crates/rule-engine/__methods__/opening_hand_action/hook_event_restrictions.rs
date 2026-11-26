use crate::HAND;
use crate::events;
use crate::rules;

pub fn hook_event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::opening_hand_action::FROM_OPENING_HAND.restriction(
            rules::in_zone(HAND, rules::Hooked::<events::OpeningHandAction>::PLAYER)
                .of(rules::Hooked::<events::OpeningHandAction>::CARD),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::events;
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
            rules::in_zone(HAND, rules::Hooked::<events::OpeningHandAction>::PLAYER,)
                .of(rules::Hooked::<events::OpeningHandAction>::CARD)
        );
    }
}
