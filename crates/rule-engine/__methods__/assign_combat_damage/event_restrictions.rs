use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![events::assign_combat_damage::ONLY_BLOCKERS.restriction(
        "A blocked creature assigns its combat damage to the creatures blocking it",
        rules::points_to::<cards::Blocking>(rules::Role::Other),
    )]
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::event_restrictions;

    #[test]
    fn test_usage() {
        let restrictions = event_restrictions();
        assert_eq!(restrictions.len(), 1);
        assert_eq!(
            restrictions[0].id,
            rules::RestrictionId("510.1c/only-blockers")
        );
        assert_eq!(
            restrictions[0].condition,
            rules::points_to::<cards::Blocking>(rules::Role::Other)
        );
    }
}
