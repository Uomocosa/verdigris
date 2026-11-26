use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::declare_attacker::CREATURE.restriction(
            "The active player chooses which creatures that they control",
            rules::Role::Subject,
            rules::has::<cards::Creature>(),
        ),
        events::declare_attacker::CONTROLS.restriction(
            "creatures that they control",
            rules::Role::Subject,
            rules::points_to::<cards::Controller>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>(),
        ),
        events::declare_attacker::UNTAPPED.restriction(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        ),
        events::declare_attacker::SUMMONING_SICK.restriction(
            "have been controlled by the active player continuously since the turn began",
            rules::Role::Subject,
            !rules::has::<cards::SummoningSick>(),
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
        assert_eq!(restrictions[0].id, rules::RestrictionId("508.1a/creature"));
        assert_eq!(restrictions[0].condition, rules::has::<cards::Creature>());
        assert_eq!(restrictions[1].id, rules::RestrictionId("508.1a/controls"));
        assert_eq!(
            restrictions[1].condition,
            rules::points_to::<cards::Controller>(rules::Role::Player)
                & rules::has::<cards::OnBattlefield>()
        );
        assert_eq!(restrictions[2].id, rules::RestrictionId("508.1a/untapped"));
        assert_eq!(restrictions[2].condition, !rules::has::<cards::Tapped>());
        assert_eq!(
            restrictions[3].id,
            rules::RestrictionId("508.1a/controlled-since-turn-began")
        );
        assert_eq!(
            restrictions[3].condition,
            !rules::has::<cards::SummoningSick>()
        );
    }
}
