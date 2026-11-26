use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::declare_attacker::CREATURE.restriction(
            "The active player chooses which creatures that they control",
            rules::has::<cards::Creature>().of(events::DeclareAttacker::ATTACKER),
        ),
        events::declare_attacker::CONTROLS.restriction(
            "creatures that they control",
            (rules::points_to::<cards::Controller>(events::DeclareAttacker::PLAYER)
                & rules::has::<cards::OnBattlefield>())
            .of(events::DeclareAttacker::ATTACKER),
        ),
        events::declare_attacker::UNTAPPED.restriction(
            "The chosen creatures must be untapped",
            (!rules::has::<cards::Tapped>()).of(events::DeclareAttacker::ATTACKER),
        ),
        events::declare_attacker::SUMMONING_SICK.restriction(
            "have been controlled by the active player continuously since the turn began",
            (!rules::has::<cards::SummoningSick>()).of(events::DeclareAttacker::ATTACKER),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::events;
    use crate::rules;

    use super::event_restrictions;

    #[test]
    fn test_usage() {
        let restrictions = event_restrictions();
        assert_eq!(restrictions.len(), 4);
        assert_eq!(restrictions[0].id, rules::RestrictionId("508.1a/creature"));
        assert_eq!(
            restrictions[0].condition,
            rules::has::<cards::Creature>().of(events::DeclareAttacker::ATTACKER)
        );
        assert_eq!(restrictions[1].id, rules::RestrictionId("508.1a/controls"));
        assert_eq!(
            restrictions[1].condition,
            (rules::points_to::<cards::Controller>(events::DeclareAttacker::PLAYER)
                & rules::has::<cards::OnBattlefield>())
            .of(events::DeclareAttacker::ATTACKER)
        );
        assert_eq!(restrictions[2].id, rules::RestrictionId("508.1a/untapped"));
        assert_eq!(
            restrictions[2].condition,
            (!rules::has::<cards::Tapped>()).of(events::DeclareAttacker::ATTACKER)
        );
        assert_eq!(
            restrictions[3].id,
            rules::RestrictionId("508.1a/controlled-since-turn-began")
        );
        assert_eq!(
            restrictions[3].condition,
            (!rules::has::<cards::SummoningSick>()).of(events::DeclareAttacker::ATTACKER)
        );
    }
}
