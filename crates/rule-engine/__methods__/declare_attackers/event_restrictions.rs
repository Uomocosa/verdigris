use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::declare_attackers::CREATURE
            .restriction(rules::has::<cards::Creature>().of(events::DeclareAttackers::ATTACKERS)),
        events::declare_attackers::CONTROLS.restriction(
            rules::has::<cards::OnBattlefield>().of(events::DeclareAttackers::ATTACKERS),
        ),
        events::declare_attackers::UNTAPPED
            .restriction((!rules::has::<cards::Tapped>()).of(events::DeclareAttackers::ATTACKERS)),
        events::declare_attackers::SUMMONING_SICK.restriction(
            (!rules::has::<cards::SummoningSick>()).of(events::DeclareAttackers::ATTACKERS),
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
            rules::has::<cards::Creature>().of(events::DeclareAttackers::ATTACKERS)
        );
        assert_eq!(restrictions[1].id, rules::RestrictionId("508.1a/controls"));
        assert_eq!(
            restrictions[1].condition,
            rules::has::<cards::OnBattlefield>().of(events::DeclareAttackers::ATTACKERS)
        );
        assert_eq!(restrictions[2].id, rules::RestrictionId("508.1a/untapped"));
        assert_eq!(
            restrictions[2].condition,
            (!rules::has::<cards::Tapped>()).of(events::DeclareAttackers::ATTACKERS)
        );
        assert_eq!(
            restrictions[3].id,
            rules::RestrictionId("508.1a/controlled-since-turn-began")
        );
        assert_eq!(
            restrictions[3].condition,
            (!rules::has::<cards::SummoningSick>()).of(events::DeclareAttackers::ATTACKERS)
        );
    }
}
