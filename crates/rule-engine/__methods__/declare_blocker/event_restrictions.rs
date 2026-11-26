use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::declare_blocker::CREATURE.restriction(
            "The defending player chooses which creatures they control",
            rules::has::<cards::Creature>(),
        ),
        events::declare_blocker::CONTROLS.restriction(
            "creatures they control",
            rules::points_to::<cards::Controller>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>(),
        ),
        events::declare_blocker::UNTAPPED.restriction(
            "The chosen creatures must be untapped",
            !rules::has::<cards::Tapped>(),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::event_restrictions;

    #[test]
    fn test_usage() {
        let restrictions = event_restrictions();
        assert_eq!(restrictions.len(), 3);
        assert_eq!(restrictions[0].id, rules::RestrictionId("509.1a/creature"));
        assert_eq!(restrictions[0].condition, rules::has::<cards::Creature>());
        assert_eq!(restrictions[1].id, rules::RestrictionId("509.1a/controls"));
        assert_eq!(
            restrictions[1].condition,
            rules::points_to::<cards::Controller>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>()
        );
        assert_eq!(restrictions[2].id, rules::RestrictionId("509.1a/untapped"));
        assert_eq!(restrictions[2].condition, !rules::has::<cards::Tapped>());
    }
}
