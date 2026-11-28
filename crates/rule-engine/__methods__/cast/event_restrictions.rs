use crate::HAND;
use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::cast::PRIORITY.restriction(rules::has_priority(events::Cast::CASTER)),
        events::cast::FROM_HAND
            .restriction(rules::in_zone(HAND, events::Cast::CASTER).of(events::Cast::SPELL)),
        events::cast::OWNS.restriction(
            rules::points_to::<cards::Owner>(events::Cast::CASTER).of(events::Cast::SPELL),
        ),
        events::cast::NOT_A_LAND
            .restriction((!rules::card_type(cards::CardType::Land)).of(events::Cast::SPELL))
            .unwaivable(),
        events::cast::TIMING.restriction(
            rules::card_type(cards::CardType::Instant).of(events::Cast::SPELL)
                | (rules::main_phase()
                    & rules::stack_empty()
                    & rules::active_player(events::Cast::CASTER)),
        ),
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
        assert_eq!(restrictions.len(), 5);
        assert_eq!(restrictions[0].id, events::cast::PRIORITY);
        assert_eq!(
            restrictions[0].condition,
            rules::has_priority(events::Cast::CASTER)
        );
        assert_eq!(restrictions[1].id, events::cast::FROM_HAND);
        assert_eq!(
            restrictions[1].condition,
            rules::in_zone(HAND, events::Cast::CASTER).of(events::Cast::SPELL)
        );
        assert_eq!(restrictions[2].id, events::cast::OWNS);
        assert_eq!(
            restrictions[2].condition,
            rules::points_to::<cards::Owner>(events::Cast::CASTER).of(events::Cast::SPELL)
        );
        assert_eq!(restrictions[3].id, events::cast::NOT_A_LAND);
        assert_eq!(
            restrictions[3].condition,
            (!rules::card_type(cards::CardType::Land)).of(events::Cast::SPELL)
        );
        assert!(restrictions[3].unwaivable);
        assert_eq!(restrictions[4].id, events::cast::TIMING);
        assert_eq!(
            restrictions[4].condition,
            rules::card_type(cards::CardType::Instant).of(events::Cast::SPELL)
                | (rules::main_phase()
                    & rules::stack_empty()
                    & rules::active_player(events::Cast::CASTER))
        );
    }
}
