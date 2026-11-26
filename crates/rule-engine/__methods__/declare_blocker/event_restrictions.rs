use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::declare_blocker::CREATURE.restriction(
            "The defending player chooses which creatures they control",
            rules::Role::Subject,
            rules::has::<cards::Creature>(),
        ),
        events::declare_blocker::CONTROLS.restriction(
            "creatures they control",
            rules::Role::Subject,
            rules::points_to::<cards::Controller>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>(),
        ),
        events::declare_blocker::UNTAPPED.restriction(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        ),
        events::declare_blocker::FLYING.restriction(
            "can’t be blocked except by creatures with flying and/or reach",
            rules::Role::Other,
            !rules::other(rules::has::<cards::Flying>())
                | rules::has::<cards::Flying>()
                | rules::has::<cards::Reach>(),
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
        assert_eq!(restrictions.len(), 4);
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
        assert_eq!(restrictions[3].id, rules::RestrictionId("702.9b/flying"));
        assert_eq!(
            restrictions[3].condition,
            !rules::other(rules::has::<cards::Flying>())
                | rules::has::<cards::Flying>()
                | rules::has::<cards::Reach>()
        );
    }
}
