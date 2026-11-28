use crate::cards;
use crate::events;
use crate::rules;

pub fn event_restrictions() -> Vec<rules::Restriction> {
    vec![
        events::declare_blocker::CREATURE
            .restriction(rules::has::<cards::Creature>().of(events::DeclareBlocker::BLOCKER)),
        events::declare_blocker::CONTROLS
            .restriction(rules::has::<cards::OnBattlefield>().of(events::DeclareBlocker::BLOCKER)),
        events::declare_blocker::UNTAPPED
            .restriction((!rules::has::<cards::Tapped>()).of(events::DeclareBlocker::BLOCKER)),
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
        assert_eq!(restrictions[0].id, events::declare_blocker::CREATURE);
        assert_eq!(
            restrictions[0].condition,
            rules::has::<cards::Creature>().of(events::DeclareBlocker::BLOCKER)
        );
        assert_eq!(restrictions[1].id, events::declare_blocker::CONTROLS);
        assert_eq!(
            restrictions[1].condition,
            rules::has::<cards::OnBattlefield>().of(events::DeclareBlocker::BLOCKER)
        );
        assert_eq!(restrictions[2].id, events::declare_blocker::UNTAPPED);
        assert_eq!(
            restrictions[2].condition,
            (!rules::has::<cards::Tapped>()).of(events::DeclareBlocker::BLOCKER)
        );
    }
}
