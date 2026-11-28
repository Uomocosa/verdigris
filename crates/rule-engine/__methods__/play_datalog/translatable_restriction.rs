use crate::rules;
use crate::rules::traits::event::Event;
use frogmyr::Atom;

use crate::legality;

pub fn translatable_restriction<E: Event>(
    event: &E,
    restriction: &rules::Restriction,
    card: &rules::ArgRef,
    holder: &rules::ArgRef,
    candidate: &Atom,
) -> bool {
    let mut scratch = Vec::new();
    let mut scratch_next = 0;
    legality::PlayDatalog::translate_holds(
        &restriction.condition.bind(event),
        card,
        holder,
        legality::PlayDatalog::CARD_VAR,
        candidate,
        &mut scratch,
        &mut scratch_next,
    )
    .is_ok()
}

#[cfg(test)]
mod tests {
    use crate::events;
    use crate::rules;
    use crate::turns;
    use bevy::prelude::Entity;

    use super::translatable_restriction;
    use crate::testing;

    #[test]
    fn test_usage() {
        let event = events::DeclareBlocker {
            blocker: Entity::PLACEHOLDER,
            attacker: Entity::PLACEHOLDER,
        };
        let card = rules::ArgRef::Arg(events::DeclareBlocker::BLOCKER);
        let holder = rules::ArgRef::Arg(events::DeclareBlocker::ATTACKER);
        let candidate = testing::block_candidate();
        let always = rules::RestrictionId("test/usage").restriction(!rules::Condition::True);
        assert!(translatable_restriction(
            &event, &always, &card, &holder, &candidate
        ));
        let step =
            rules::RestrictionId("test/unusable").restriction(rules::step_is(turns::Step::Upkeep));
        assert!(!translatable_restriction(
            &event, &step, &card, &holder, &candidate
        ));
    }
}
