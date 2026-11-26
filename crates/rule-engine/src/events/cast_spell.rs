use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const PRIORITY: rules::RestrictionId = rules::RestrictionId("601.2a/priority");
pub const FROM_HAND: rules::RestrictionId = rules::RestrictionId("601.2a/from-hand");
pub const OWNS: rules::RestrictionId = rules::RestrictionId("601.2a/owns");
pub const NOT_A_LAND: rules::RestrictionId = rules::RestrictionId("601.2a/not-a-land");
pub const TIMING: rules::RestrictionId = rules::RestrictionId("601.2a/timing");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CastSpell {
    pub spell: Entity,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl rules::traits::context::Context for CastSpell {
    fn subject(&self) -> Entity { self.spell }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

impl Event for CastSpell {
    const ID: rules::EventId = rules::EventId("601.2a/cast-spell");

    #[atomic_delegate(CastSpell, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::CastSpell;
    use super::FROM_HAND;
    use super::NOT_A_LAND;
    use super::OWNS;
    use super::PRIORITY;
    use super::TIMING;

    #[test]
    fn test_usage() {
        assert_eq!(CastSpell::ID, rules::EventId("601.2a/cast-spell"));
        let restrictions = CastSpell::restrictions();
        assert_eq!(restrictions.len(), 5);
        assert_eq!(restrictions[0].id, PRIORITY);
        assert_eq!(restrictions[0].text, "A player who has priority");
        assert_eq!(
            restrictions[0].condition,
            rules::has_priority(rules::Role::Player)
        );
        assert_eq!(restrictions[1].id, FROM_HAND);
        assert_eq!(restrictions[1].text, "from their hand");
        assert_eq!(
            restrictions[1].condition,
            rules::in_zone(HAND, rules::Role::Player)
        );
        assert_eq!(restrictions[2].id, OWNS);
        assert_eq!(restrictions[2].text, "their hand");
        assert_eq!(
            restrictions[2].condition,
            rules::points_to::<cards::Owner>(rules::Role::Player)
        );
        assert_eq!(restrictions[3].id, NOT_A_LAND);
        assert_eq!(restrictions[3].text, "It can’t be cast as a spell.");
        assert_eq!(
            restrictions[3].condition,
            !rules::card_type(cards::CardType::Land)
        );
        assert!(restrictions[3].unwaivable);
        assert_eq!(restrictions[4].id, TIMING);
        assert_eq!(
            restrictions[4].text,
            "during a main phase of their turn when the stack is empty"
        );
        assert_eq!(
            restrictions[4].condition,
            rules::card_type(cards::CardType::Instant)
                | (rules::main_phase()
                    & rules::stack_empty()
                    & rules::active_player(rules::Role::Player))
        );
        assert!(
            CastSpell::restrictions()
                .iter()
                .filter(|restriction| restriction.id != NOT_A_LAND)
                .all(|restriction| !restriction.unwaivable)
        );
    }

    #[test]
    fn restriction_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        for restriction in CastSpell::restrictions() {
            assert!(
                RULES_TEXT.contains(restriction.text),
                "restriction {:?} text not found verbatim in Comprehensive Rules: {:?}",
                restriction.id,
                restriction.text
            );
        }
    }
}
