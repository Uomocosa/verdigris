use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::declare_blocker::CREATURE.restriction(
            "The defending player chooses which creatures they control",
            rules::has::<cards::Creature>().of(events::DeclareBlocker::BLOCKER),
        ),
        events::declare_blocker::CONTROLS.restriction(
            "creatures they control",
            (rules::points_to::<cards::Controller>(events::DeclareBlocker::PLAYER)
                & rules::has::<cards::OnBattlefield>())
            .of(events::DeclareBlocker::BLOCKER),
        ),
        events::declare_blocker::UNTAPPED.restriction(
            "The chosen creatures must be untapped",
            (!rules::has::<cards::Tapped>()).of(events::DeclareBlocker::BLOCKER),
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
        assert_eq!(restrictions.len(), 3);
        assert_eq!(restrictions[0].id, rules::RestrictionId("509.1a/creature"));
        assert_eq!(
            restrictions[0].condition,
            rules::has::<cards::Creature>().of(events::DeclareBlocker::BLOCKER)
        );
        assert_eq!(restrictions[1].id, rules::RestrictionId("509.1a/controls"));
        assert_eq!(
            restrictions[1].condition,
            (rules::points_to::<cards::Controller>(events::DeclareBlocker::PLAYER)
                & rules::has::<cards::OnBattlefield>())
            .of(events::DeclareBlocker::BLOCKER)
        );
        assert_eq!(restrictions[2].id, rules::RestrictionId("509.1a/untapped"));
        assert_eq!(
            restrictions[2].condition,
            (!rules::has::<cards::Tapped>()).of(events::DeclareBlocker::BLOCKER)
        );
    }
}
